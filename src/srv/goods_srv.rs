use std::sync::Arc;

use super::{err_to_tonic_status as ce2s, sqlx_err_to_tonic_status as e2s};

use crate::{
    db, model,
    pb::{self, goods_service_server::GoodsService},
};

pub struct Goods {
    pool: Arc<sqlx::PgPool>,
}

impl Goods {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self {
            pool: Arc::new(pool),
        }
    }
}

#[tonic::async_trait]
impl GoodsService for Goods {
    /// 创建品牌
    async fn create_brand(
        &self,
        request: tonic::Request<pb::Brand>,
    ) -> std::result::Result<tonic::Response<pb::Id>, tonic::Status> {
        let r = request.into_inner();
        let m = model::Brand::from(r);

        let is_exists = self
            .brand_exists(tonic::Request::new(pb::BrandExistsRequest {
                name: m.name.clone(),
                id: None,
            }))
            .await?
            .into_inner();
        if is_exists.value {
            return Err(tonic::Status::already_exists("品牌已存在"));
        };
        let id = db::brand::create(&self.pool, &m).await.map_err(ce2s)?;

        Ok(tonic::Response::new(pb::Id { value: id }))
    }
    /// 修改品牌
    async fn edit_brand(
        &self,
        request: tonic::Request<pb::Brand>,
    ) -> std::result::Result<tonic::Response<pb::Aff>, tonic::Status> {
        let r = request.into_inner();
        let id = r.id.clone();

        if (*id).is_empty() {
            return Err(tonic::Status::invalid_argument("请指定ID"));
        }

        let m = model::Brand::from(r);

        let is_exists = self
            .brand_exists(tonic::Request::new(pb::BrandExistsRequest {
                name: m.name.clone(),
                id: Some(id),
            }))
            .await?
            .into_inner();
        if is_exists.value {
            return Err(tonic::Status::already_exists("品牌已存在"));
        }

        let rows = db::brand::edit(&self.pool, &m).await.map_err(ce2s)?;

        Ok(tonic::Response::new(pb::Aff { rows }))
    }
    /// 删除或还原品牌
    async fn delete_or_restore_brand(
        &self,
        request: tonic::Request<pb::DeleteOrRestoreRequest>,
    ) -> std::result::Result<tonic::Response<pb::Aff>, tonic::Status> {
        let r = request.into_inner();
        let rows = db::brand::del_or_restore(&self.pool, r.id, r.is_del)
            .await
            .map_err(ce2s)?;
        Ok(tonic::Response::new(pb::Aff { rows }))
    }
    /// 查找品牌
    async fn find_brand(
        &self,
        request: tonic::Request<pb::FindBrandRequest>,
    ) -> std::result::Result<tonic::Response<pb::FindBrandResponse>, tonic::Status> {
        let r = request.into_inner();
        let r = model::BrandFindRequest::from(r);
        let brand = db::brand::find(&self.pool, &r).await.map_err(ce2s)?;
        let brand: Option<pb::Brand> = match brand {
            Some(b) => Some(b.into()),
            None => None,
        };
        Ok(tonic::Response::new(pb::FindBrandResponse { brand }))
    }
    /// 品牌列表
    async fn list_brand(
        &self,
        request: tonic::Request<pb::ListBrandRequest>,
    ) -> std::result::Result<tonic::Response<pb::ListBrandResponse>, tonic::Status> {
        let r = model::BrandListRequest::from(request.into_inner());
        let rs = db::brand::list(&self.pool, &r).await.map_err(ce2s)?;
        let paginate = rs.to_pb();

        let mut brands = Vec::with_capacity(rs.data.len());
        for b in rs.data {
            brands.push(b.into());
        }
        Ok(tonic::Response::new(pb::ListBrandResponse {
            paginate: Some(paginate),
            brands,
        }))
    }
    /// 品牌是否存在
    async fn brand_exists(
        &self,
        request: tonic::Request<pb::BrandExistsRequest>,
    ) -> std::result::Result<tonic::Response<pb::IsExistsResponse>, tonic::Status> {
        let r = request.into_inner();
        let rs = db::brand::exists(&self.pool, &r.name, r.id)
            .await
            .map_err(ce2s)?;
        Ok(tonic::Response::new(pb::IsExistsResponse { value: rs }))
    }
    /// 创建分类
    async fn create_category(
        &self,
        request: tonic::Request<pb::Category>,
    ) -> std::result::Result<tonic::Response<pb::Id>, tonic::Status> {
        let cate = model::Category::from(request.into_inner());
        let exists_req = model::CategoryExistsRequest {
            name_and_parent: model::CategoryNameAndParentRequest {
                name: cate.name.clone(),
                parent: cate.parent.clone(),
            },
            id: None,
        };

        let mut tx = self.pool.begin().await.map_err(e2s)?;

        let exists = match db::category::exists(&mut *tx, &exists_req).await {
            Ok(exists) => exists,
            Err(e) => {
                tx.rollback().await.map_err(e2s)?;
                return Err(e2s(e));
            }
        };
        if exists {
            tx.rollback().await.map_err(e2s)?;
            return Err(tonic::Status::already_exists("同名的分类已存在"));
        }

        let id = match db::category::create(&mut *tx, &cate).await {
            Ok(id) => id,
            Err(e) => {
                tx.rollback().await.map_err(e2s)?;
                return Err(e2s(e));
            }
        };
        tx.commit().await.map_err(e2s)?;

        Ok(tonic::Response::new(pb::Id { value: id }))
    }
    /// 修改分类
    async fn edit_category(
        &self,
        request: tonic::Request<pb::Category>,
    ) -> std::result::Result<tonic::Response<pb::Aff>, tonic::Status> {
        let cate = model::Category::from(request.into_inner());
        let mut tx = self.pool.begin().await.map_err(e2s)?;

        if let Err(e) = db::category::edit(&mut *tx, &cate).await {
            tx.rollback().await.map_err(e2s)?;
            return Err(e2s(e));
        }

        // 子分类
        let children = match db::category::get_children(&mut *tx, &cate.path, &cate.id).await {
            Ok(children) => children,
            Err(e) => {
                tx.rollback().await.map_err(e2s)?;
                return Err(e2s(e));
            }
        };

        for c in children.iter() {
            if let Err(e) = db::category::edit(&mut *tx, c).await {
                tx.rollback().await.map_err(e2s)?;
                return Err(e2s(e));
            }
        }

        tx.commit().await.map_err(e2s)?;
        unimplemented!()
    }
    /// 修改分类名称
    async fn edit_category_name(
        &self,
        request: tonic::Request<pb::Category>,
    ) -> std::result::Result<tonic::Response<pb::Aff>, tonic::Status> {
        let cate = model::Category::from(request.into_inner());
        let exists_req = model::CategoryExistsRequest {
            name_and_parent: model::CategoryNameAndParentRequest {
                name: cate.name.clone(),
                parent: cate.parent.clone(),
            },
            id: Some(cate.id.clone()),
        };
        let mut tx = self.pool.begin().await.map_err(e2s)?;

        let exists = match db::category::exists(&mut *tx, &exists_req).await {
            Ok(exists) => exists,
            Err(e) => {
                tx.rollback().await.map_err(e2s)?;
                return Err(e2s(e));
            }
        };
        if exists {
            tx.rollback().await.map_err(e2s)?;
            return Err(tonic::Status::already_exists("同名的分类已存在"));
        }

        let rows = db::category::edit_name(&mut *tx, &cate.id, &cate.name)
            .await
            .map_err(e2s)?;

        tx.commit().await.map_err(e2s)?;
        Ok(tonic::Response::new(pb::Aff { rows }))
    }
    /// 删除或还原分类
    async fn delete_or_restore_category(
        &self,
        request: tonic::Request<pb::DeleteOrRestoreRequest>,
    ) -> std::result::Result<tonic::Response<pb::Aff>, tonic::Status> {
        let req = request.into_inner();
        let rows = db::category::del_or_restore(&self.pool, req.id, req.is_del)
            .await
            .map_err(ce2s)?;
        Ok(tonic::Response::new(pb::Aff { rows }))
    }
    /// 分类是否存在
    async fn category_exists(
        &self,
        request: tonic::Request<pb::CategoryExistsRequest>,
    ) -> std::result::Result<tonic::Response<pb::IsExistsResponse>, tonic::Status> {
        let r = request.into_inner();
        {
            let r_check = r.name_and_parent.clone();
            let e_check = Err(tonic::Status::invalid_argument(
                "参数错误：请至少指定分类名称；如果父分类为空，则视为一级分类",
            ));
            if r_check.is_none() {
                return e_check;
            }
            if r_check.unwrap().name.is_empty() {
                return e_check;
            }
        }
        let r = model::CategoryExistsRequest::from(r);
        let mut tx = self.pool.begin().await.map_err(e2s)?;
        let exists = match db::category::exists(&mut *tx, &r).await {
            Err(err) => {
                tx.rollback().await.map_err(e2s)?;

                return Err(e2s(err));
            }
            Ok(exists) => exists,
        };
        tx.commit().await.map_err(e2s)?;
        Ok(tonic::Response::new(pb::IsExistsResponse { value: exists }))
    }
    /// 查找分类
    async fn find_category(
        &self,
        request: tonic::Request<pb::FindCategoryRequest>,
    ) -> std::result::Result<tonic::Response<pb::FindCategoryResponse>, tonic::Status> {
        let r: model::FindCategoryRequest = request.into_inner().into();
        let cate = db::category::find(&self.pool, &r).await.map_err(ce2s)?;
        let cate: Option<pb::Category> = match cate {
            Some(c) => Some(c.into()),
            None => None,
        };
        Ok(tonic::Response::new(pb::FindCategoryResponse {
            category: cate,
        }))
    }
    /// 分类列表
    async fn list_category(
        &self,
        request: tonic::Request<pb::ListCategoryRequest>,
    ) -> std::result::Result<tonic::Response<pb::ListCategoryResponse>, tonic::Status> {
        let r = model::ListCategoryRequest::from(request.into_inner());
        let rs = db::category::list(&self.pool, &r).await.map_err(ce2s)?;
        let paginate = rs.to_pb();

        let mut categories = Vec::with_capacity(rs.data.len());
        for c in rs.data {
            categories.push(c.into());
        }
        Ok(tonic::Response::new(pb::ListCategoryResponse {
            paginate: Some(paginate),
            categoies: categories,
        }))
    }
    /// 分类树
    async fn category_tree(
        &self,
        request: tonic::Request<pb::CategoryTreeRequest>,
    ) -> std::result::Result<tonic::Response<pb::CategoryTreeResponse>, tonic::Status> {
        let r = model::CategoryTreeRequest::from(request.into_inner());
        let tree_db = db::category::tree(&*self.pool, &r).await.map_err(e2s)?;

        let mut tree = Vec::with_capacity(tree_db.len());
        for t in tree_db {
            tree.push(t.into());
        }

        Ok(tonic::Response::new(pb::CategoryTreeResponse { tree }))
    }
    /// 查找带品牌信息的分类
    async fn find_category_with_brands(
        &self,
        request: tonic::Request<pb::FindCategoryWithBrandsRequest>,
    ) -> std::result::Result<tonic::Response<pb::FindCategoryWithBrandsResponse>, tonic::Status>
    {
        let r = model::FindCategoryWithBrandsRequest::from(request.into_inner());
        let cate = db::category_brand::find_category(&*self.pool, &r)
            .await
            .map_err(e2s)?;
        let cate = match cate {
            Some(c) => Some(c.into()),
            None => None,
        };

        Ok(tonic::Response::new(pb::FindCategoryWithBrandsResponse {
            category: cate,
        }))
    }
    /// 带品牌信息分类列表
    async fn list_category_with_brands(
        &self,
        request: tonic::Request<pb::ListCategoryWithBrandsRequest>,
    ) -> std::result::Result<tonic::Response<pb::ListCategoryWithBrandsResponse>, tonic::Status>
    {
        let r = model::ListCategoryWithBrandsRequest::from(request.into_inner());
        let rs = db::category_brand::list_with_brands(&self.pool, &r)
            .await
            .map_err(ce2s)?;
        let paginate = rs.to_pb();

        let mut categories = Vec::with_capacity(rs.data.len());
        for c in rs.data {
            categories.push(c.into());
        }
        Ok(tonic::Response::new(pb::ListCategoryWithBrandsResponse {
            paginate: Some(paginate),
            categoies: categories,
        }))
    }
    /// 带品牌信息分类树
    async fn category_with_brands_tree(
        &self,
        request: tonic::Request<pb::CategoryTreeRequest>,
    ) -> std::result::Result<tonic::Response<pb::CategoryWithBrandsTreeResponse>, tonic::Status>
    {
        let r = model::CategoryTreeRequest::from(request.into_inner());
        let tree_db = db::category_brand::tree(&*self.pool, &r)
            .await
            .map_err(e2s)?;

        let mut tree = Vec::with_capacity(tree_db.len());
        for t in tree_db {
            tree.push(t.into());
        }

        Ok(tonic::Response::new(pb::CategoryWithBrandsTreeResponse {
            tree,
        }))
    }
    /// 查找带分类信息的品牌
    async fn find_brand_with_categoies(
        &self,
        request: tonic::Request<pb::FindBrandWithCategoiesRequest>,
    ) -> std::result::Result<tonic::Response<pb::FindBrandWithCategoiesResponse>, tonic::Status>
    {
        let r = model::FindBrandWithCategoriesRequest::from(request.into_inner());
        let brand = db::category_brand::find_brand(&*self.pool, &r)
            .await
            .map_err(e2s)?;
        let brand = match brand {
            Some(b) => Some(b.into()),
            None => None,
        };
        Ok(tonic::Response::new(pb::FindBrandWithCategoiesResponse {
            brand,
        }))
    }
    /// 带分类信息品牌列表
    async fn list_brand_with_categoies(
        &self,
        request: tonic::Request<pb::ListBrandWithCategoiesRequest>,
    ) -> std::result::Result<tonic::Response<pb::ListBrandWithCategoiesResponse>, tonic::Status>
    {
        let r = model::ListBrandWithCategoriesRequest::from(request.into_inner());
        let rs = db::category_brand::list_with_categoies(&self.pool, &r)
            .await
            .map_err(ce2s)?;
        let paginate = rs.to_pb();

        let mut brands = Vec::with_capacity(rs.data.len());
        for b in rs.data {
            brands.push(b.into());
        }

        Ok(tonic::Response::new(pb::ListBrandWithCategoiesResponse {
            paginate: Some(paginate),
            brands,
        }))
    }
    /// 设置分类-品牌
    async fn set_category_brands(
        &self,
        request: tonic::Request<pb::SetCategoryBrandsRequest>,
    ) -> std::result::Result<tonic::Response<pb::Aff>, tonic::Status> {
        let r = model::SetCategoryBrandRequest::from(request.into_inner());
        let cr = model::ClearCategoryBrandsRequest {
            category_id: r.category_id.clone(),
        };
        let mut tx = self.pool.begin().await.map_err(e2s)?;
        if let Err(e) = db::category_brand::clear(&mut *tx, &cr).await {
            tx.rollback().await.map_err(e2s)?;
            return Err(e2s(e));
        };
        let aff = match db::category_brand::set(&mut *tx, &r).await {
            Ok(aff) => aff,
            Err(e) => {
                tx.rollback().await.map_err(e2s)?;
                return Err(e2s(e));
            }
        };
        tx.commit().await.map_err(e2s)?;
        Ok(tonic::Response::new(pb::Aff { rows: aff }))
    }
    /// 清空分类的品牌
    async fn clear_category_brands(
        &self,
        request: tonic::Request<pb::ClearCategoryBrandsRequest>,
    ) -> std::result::Result<tonic::Response<pb::Aff>, tonic::Status> {
        let r = model::ClearCategoryBrandsRequest::from(request.into_inner());
        let aff = db::category_brand::clear(&*self.pool, &r)
            .await
            .map_err(e2s)?;
        Ok(tonic::Response::new(pb::Aff { rows: aff }))
    }
    /// 创建带品牌的分类
    async fn create_category_with_brands(
        &self,
        request: tonic::Request<pb::CreateCategoryWithBrandsRequest>,
    ) -> std::result::Result<tonic::Response<pb::CreateCategoryWithBrandsResponse>, tonic::Status>
    {
        let r = request.into_inner();
        let cate = model::Category::from(r.category.unwrap());

        let exists_req = model::CategoryExistsRequest {
            name_and_parent: model::CategoryNameAndParentRequest {
                name: cate.name.clone(),
                parent: cate.parent.clone(),
            },
            id: None,
        };

        let mut tx = self.pool.begin().await.map_err(e2s)?;

        let exists = match db::category::exists(&mut *tx, &exists_req).await {
            Ok(exists) => exists,
            Err(e) => {
                tx.rollback().await.map_err(e2s)?;
                return Err(e2s(e));
            }
        };
        if exists {
            tx.rollback().await.map_err(e2s)?;
            return Err(tonic::Status::already_exists("同名的分类已存在"));
        }

        let id = match db::category::create(&mut *tx, &cate).await {
            Ok(id) => id,
            Err(e) => {
                tx.rollback().await.map_err(e2s)?;
                return Err(e2s(e));
            }
        };

        let set_r = model::SetCategoryBrandRequest {
            category_id: id.clone(),
            brand_ids: r.brand_ids,
        };

        let aff = match db::category_brand::set(&mut *tx, &set_r).await {
            Ok(aff) => aff,
            Err(e) => {
                tx.rollback().await.map_err(e2s)?;
                return Err(e2s(e));
            }
        };
        tx.commit().await.map_err(e2s)?;

        Ok(tonic::Response::new(pb::CreateCategoryWithBrandsResponse {
            id: Some(pb::Id { value: id }),
            aff: Some(pb::Aff { rows: aff }),
        }))
    }

    /// 创建商品
    async fn create_goods(
        &self,
        request: tonic::Request<pb::Goods>,
    ) -> std::result::Result<tonic::Response<pb::Id>, tonic::Status> {
        let g = model::Goods::from(request.into_inner());
        let mut tx = self.pool.begin().await.map_err(e2s)?;

        let name_exists = match db::goods::exists(
            &mut *tx,
            &model::GoodsExistsRequest {
                by: model::GoodsExistsBy::Name(g.name.clone()),
                id: None,
            },
        )
        .await
        {
            Err(e) => {
                tx.rollback().await.map_err(e2s)?;
                return Err(e2s(e));
            }
            Ok(e) => e,
        };
        let sn_exists = match db::goods::exists(
            &mut *tx,
            &model::GoodsExistsRequest {
                by: model::GoodsExistsBy::SN(g.sn.clone()),
                id: None,
            },
        )
        .await
        {
            Err(e) => {
                tx.rollback().await.map_err(e2s)?;
                return Err(e2s(e));
            }
            Ok(e) => e,
        };

        if name_exists {
            return Err(tonic::Status::already_exists("同名的商品已经存在"));
        }

        if sn_exists {
            return Err(tonic::Status::already_exists("相同的商品分类已经存在"));
        }

        let id = match db::goods::create(&*self.pool, &g).await {
            Ok(id) => id,
            Err(e) => {
                tx.rollback().await.map_err(e2s)?;
                return Err(e2s(e));
            }
        };

        tx.commit().await.map_err(e2s)?;
        Ok(tonic::Response::new(pb::Id { value: id }))
    }
    /// 修改商品
    async fn edit_goods(
        &self,
        request: tonic::Request<pb::Goods>,
    ) -> std::result::Result<tonic::Response<pb::Aff>, tonic::Status> {
        let g = model::Goods::from(request.into_inner());
        let mut tx = self.pool.begin().await.map_err(e2s)?;

        let name_exists = match db::goods::exists(
            &mut *tx,
            &model::GoodsExistsRequest {
                by: model::GoodsExistsBy::Name(g.name.clone()),
                id: Some(g.id.clone()),
            },
        )
        .await
        {
            Err(e) => {
                tx.rollback().await.map_err(e2s)?;
                return Err(e2s(e));
            }
            Ok(e) => e,
        };
        let sn_exists = match db::goods::exists(
            &mut *tx,
            &model::GoodsExistsRequest {
                by: model::GoodsExistsBy::SN(g.sn.clone()),
                id: Some(g.id.clone()),
            },
        )
        .await
        {
            Err(e) => {
                tx.rollback().await.map_err(e2s)?;
                return Err(e2s(e));
            }
            Ok(e) => e,
        };

        if name_exists {
            return Err(tonic::Status::already_exists("同名的商品已经存在"));
        }

        if sn_exists {
            return Err(tonic::Status::already_exists("相同的商品分类已经存在"));
        }

        let rows = match db::goods::edit(&mut *tx, &g).await {
            Ok(aff) => aff,
            Err(e) => {
                tx.rollback().await.map_err(e2s)?;
                return Err(e2s(e));
            }
        };

        Ok(tonic::Response::new(pb::Aff { rows }))
    }
    /// 删除/还原商品
    async fn delete_or_restore_goods(
        &self,
        request: tonic::Request<pb::DeleteOrRestoreRequest>,
    ) -> std::result::Result<tonic::Response<pb::Aff>, tonic::Status> {
        let r = request.into_inner();
        let rows = db::goods::del_or_restore(&self.pool, r.id, r.is_del)
            .await
            .map_err(ce2s)?;
        Ok(tonic::Response::new(pb::Aff { rows }))
    }
    /// 查找商品
    async fn find_goods(
        &self,
        request: tonic::Request<pb::FindGoodsRequest>,
    ) -> std::result::Result<tonic::Response<pb::FindGoodsResponse>, tonic::Status> {
        let r = model::FindGoodsRequest::from(request.into_inner());
        let g = db::goods::find(&self.pool, &r).await.map_err(ce2s)?;
        let goods = match g {
            Some(g) => Some(g.into()),
            None => None,
        };
        Ok(tonic::Response::new(pb::FindGoodsResponse { goods }))
    }
    /// 商品列表
    async fn list_goods(
        &self,
        request: tonic::Request<pb::ListGoodsRequest>,
    ) -> std::result::Result<tonic::Response<pb::ListGoodsResponse>, tonic::Status> {
        let r = model::ListGoodsRequest::from(request.into_inner());
        let p = db::goods::list(&self.pool, &r).await.map_err(ce2s)?;
        let paginate = p.to_pb();

        let mut goods = Vec::with_capacity(p.data.len());
        for g in p.data {
            goods.push(g.into());
        }

        Ok(tonic::Response::new(pb::ListGoodsResponse {
            paginate: Some(paginate),
            goods,
        }))
    }
    /// 商品是否存在
    async fn goods_exists(
        &self,
        request: tonic::Request<pb::GoodsExistsRequest>,
    ) -> std::result::Result<tonic::Response<pb::IsExistsResponse>, tonic::Status> {
        let r = model::GoodsExistsRequest::from(request.into_inner());
        let exists = db::goods::exists(&*self.pool, &r).await.map_err(e2s)?;
        Ok(tonic::Response::new(pb::IsExistsResponse { value: exists }))
    }
    /// 设置商品属性
    async fn set_goods_attr(
        &self,
        request: tonic::Request<pb::GoodsAttr>,
    ) -> std::result::Result<tonic::Response<pb::Aff>, tonic::Status> {
        let ga = model::GoodsAttr::from(request.into_inner());
        let mut tx = self.pool.begin().await.map_err(e2s)?;

        let rows = match db::goods_attr::set(&mut *tx, &ga).await {
            Ok(rows) => rows,
            Err(err) => {
                tx.rollback().await.map_err(e2s)?;
                return Err(e2s(err));
            }
        };

        tx.commit().await.map_err(e2s)?;

        Ok(tonic::Response::new(pb::Aff { rows }))
    }
    /// 删除商品属性
    async fn remove_goods_attr(
        &self,
        request: tonic::Request<pb::Id>,
    ) -> std::result::Result<tonic::Response<pb::Aff>, tonic::Status> {
        let rows = db::goods_attr::remove(&*self.pool, &request.into_inner().value)
            .await
            .map_err(e2s)?;
        Ok(tonic::Response::new(pb::Aff { rows }))
    }
    /// 查找商品属性
    async fn find_goods_attr(
        &self,
        request: tonic::Request<pb::Id>,
    ) -> std::result::Result<tonic::Response<pb::FindGoodsAttrResponse>, tonic::Status> {
        let ga = db::goods_attr::find(&*self.pool, &request.into_inner().value)
            .await
            .map_err(e2s)?;
        let ga = match ga {
            Some(ga) => Some(ga.into()),
            None => None,
        };

        Ok(tonic::Response::new(pb::FindGoodsAttrResponse {
            goods_attr: ga,
        }))
    }
    /// 更新商品库存
    async fn update_goods_stock(
        &self,
        request: tonic::Request<pb::UpdateGoodsStockRequest>,
    ) -> std::result::Result<tonic::Response<pb::Aff>, tonic::Status> {
        let r = model::UpdateGoodsStockRequest::from(request.into_inner());
        let mut tx = self.pool.begin().await.map_err(e2s)?;

        let ga = match db::goods_attr::find(&mut *tx, &r.goods_id).await {
            Ok(ga) => ga,
            Err(err) => {
                tx.rollback().await.map_err(e2s)?;
                return Err(e2s(err));
            }
        };
        if ga.is_none() {
            tx.rollback().await.map_err(e2s)?;
            return Err(tonic::Status::not_found("不存在的记录"));
        }

        let ga = ga.unwrap();
        let r = model::UpdateGoodsStockRequest { ver: ga.ver, ..r };

        let rows = match db::goods_attr::update_sock(&mut *tx, &r).await {
            Ok(rows) => rows,
            Err(err) => {
                tx.rollback().await.map_err(e2s)?;
                return Err(e2s(err));
            }
        };

        tx.commit().await.map_err(e2s)?;

        Ok(tonic::Response::new(pb::Aff { rows }))
    }
}

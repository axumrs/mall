CREATE OR REPLACE VIEW v_category_brands AS -- 分类与品牌
SELECT
	c.id, c."name", parent, "path", "level", c.dateline, c.is_del
	, b.id AS brand_id, b."name" AS brand_name, b.logo AS brand_logo, b.is_del AS brand_is_del, b.dateline AS brand_dateline
FROM categoies AS c
LEFT JOIN
	category_brands AS  cb 
ON 
	cb.category_id = c.id 
LEFT JOIN 
	brands AS b 
ON
	cb.brand_id  = b.id 
;

CREATE OR REPLACE VIEW v_category_with_brands AS -- 分类的品牌信息
SELECT DISTINCT  cb.id, "name", parent, "path", "level", dateline, is_del,brand_ids,brand_names,brand_logos,brand_is_dels,brand_datelines,brand_names_str FROM v_category_brands AS cb
,(SELECT id, 
	array_agg (brand_id) AS brand_ids, 
	array_agg(brand_name) AS brand_names,
	',' || COALESCE (string_agg(brand_name, ','), '') || ',' AS brand_names_str,
	array_agg(brand_logo) AS brand_logos,
	array_agg(brand_is_del) AS brand_is_dels,
	array_agg(brand_dateline) AS brand_datelines
	FROM v_category_brands GROUP BY id ORDER BY id) AS t
WHERE cb.id=t.id
;

CREATE OR REPLACE VIEW v_brand_categoies AS -- 品牌与分类
SELECT
	c.id, c."name", parent, "path", "level", c.dateline, c.is_del
	, b.id AS brand_id, b."name" AS brand_name, b.logo AS brand_logo, b.is_del AS brand_is_del, b.dateline AS brand_dateline
FROM brands AS b
LEFT JOIN
	category_brands AS  cb 
ON 
	cb.brand_id  = b.id 
LEFT JOIN 
	categoies  AS c
ON
	cb.category_id  = c.id 
;

CREATE OR REPLACE VIEW v_brand_with_categoies AS -- 品牌的分类信息
SELECT DISTINCT cb.brand_id , brand_name , brand_logo , brand_is_del , brand_dateline,ids,names,names_str,parents,levels,paths,datelines,is_dels  FROM v_brand_categoies AS cb,
(SELECT brand_id,
	array_agg(id) AS ids,
	array_agg(name) AS names,
	',' || COALESCE (string_agg(name, ','), '') || ',' AS names_str,
	array_agg(parent) AS parents,
	array_agg(level) AS levels,
	array_agg(path) AS paths,
	array_agg(dateline) AS datelines,
	array_agg(is_del) AS is_dels 
	FROM  v_brand_categoies GROUP BY brand_id) AS t
WHERE t.brand_id = cb.brand_id;
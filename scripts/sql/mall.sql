CREATE TYPE "user_status" AS ENUM('Pending', 'Actived', 'Freezed');

CREATE TABLE IF NOT EXISTS "users" ( -- 用户
    "id" CHAR(20) PRIMARY KEY,
    "email" VARCHAR(255) NOT NULL, -- 邮箱
    "password" VARCHAR(255) NOT NULL, -- 密码
    "nickname" VARCHAR(50) NOT NULL, -- 昵称
    "status" user_status NOT NULL DEFAULT 'Pending', -- 状态
    "dateline" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "is_del" BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE("email")
);

CREATE TABLE IF NOT EXISTS "brands" ( -- 品牌
    "id" CHAR(20) PRIMARY KEY,
    "name" VARCHAR(100) NOT NULL, -- 名称
    "logo" VARCHAR(255) NOT NULL, -- LOGO
    "is_del" BOOLEAN NOT NULL DEFAULT FALSE,
    "dateline" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE("name")
);

CREATE TYPE "category_level" AS ENUM('Unspecified', 'Level1', 'Level2', 'Level3');

CREATE TABLE IF NOT EXISTS "categoies" ( -- 商品分类
    "id" CHAR(20) PRIMARY KEY,
    "name" VARCHAR(100) NOT NULL, -- 名称
    "parent" CHAR(20) NOT NULL DEFAULT '', -- 父级分类
    "path" VARCHAR NOT NULL DEFAULT '', -- 路径
    "level" category_level NOT NULL DEFAULT 'Unspecified', -- 级别
    "dateline" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "is_del" BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE("name", "parent")
);

CREATE TABLE IF NOT EXISTS "category_brands" ( -- 商品分类与品牌
    "brand_id" CHAR(20) NOT NULL, -- 品牌ID
    "category_id" CHAR(20) NOT NULL, -- 分类ID
    PRIMARY KEY("brand_id", "category_id")
);


CREATE TABLE IF NOT EXISTS "banners" ( -- 轮播图
    "id" CHAR(20) PRIMARY KEY,
    "img" VARCHAR(255) NOT NULL, -- 图片地址
    "url" VARCHAR(255) NOT NULL, -- 链接地址
    "sort" INTEGER NOT NULL DEFAULT 0, -- 排序
    "title" VARCHAR(50) NOT NULL, -- 标题
    "is_del" BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS "goods" ( -- 商品
	"id" CHAR(20) PRIMARY KEY,
	"category_id" CHAR(20) NOT NULL, -- 分类ID
	"name" VARCHAR(255) NOT NULL, -- 商品名称
	"sn" VARCHAR(64) NOT NULL, -- 商品编号
	"is_sale" BOOLEAN NOT NULL DEFAULT FALSE, -- 是否上架中
	"ship_fee" u32 NOT NULL DEFAULT 0, -- 运费
	"is_new" BOOLEAN NOT NULL DEFAULT FALSE, -- 是否新品
	"is_hot" BOOLEAN NOT NULL DEFAULT FALSE, -- 是否热销
	"hit" u64 NOT NULL DEFAULT 0, -- 点击次数
	"sold_num" u64 NOT NULL DEFAULT 0, -- 销量
	"fav_num" u64 NOT NULL DEFAULT 0, -- 收藏量
	"origin_price" u32 NOT NULL, -- 原价
	"price" u32 NOT NULL, -- 销售价
	"brief" VARCHAR(255) NOT NULL DEFAULT '' , -- 简介【副标题】
	"cover" VARCHAR(255) NOT NULL DEFAULT '', -- 封面
	"images" VARCHAR(255)[] NOT NULL, -- 图片
	"description" VARCHAR NOT NULL, -- 描述
	"dateline" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"is_del" BOOLEAN NOT NULL DEFAULT FALSE,
	UNIQUE ("name"),
	UNIQUE ("sn")
);

CREATE TABLE IF NOT EXISTS "goods_attrs"( -- 商品属性
	goods_id CHAR(20) PRIMARY KEY, -- 商品ID
    sku JSONB NOT NULL DEFAULT '{}', -- 商品SKU
    ver u64 NOT NULL DEFAULT 0 -- 乐观锁版本
);

CREATE TABLE IF NOT EXISTS "carts" ( -- 购物车
	"id" CHAR(20) PRIMARY KEY,
	"user_id" CHAR(20) NOT NULL, -- 用户ID
	"goods_id" CHAR(20) NOT NULL, -- 商品ID
	"goods_sku" VARCHAR(255) NOT NULL, -- SKU
	"num" u32 NOT NULL, -- 购买数量
	"amount" u32 NOT NULL, -- 金额
	"hash" CHAR(32) NOT NULL, -- hash
	"dateline" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP, -- 加入购物车时间
	UNIQUE("hash")
);

CREATE TABLE IF NOT EXISTS "address" ( -- 收货地址
	"id" CHAR(20) PRIMARY KEY,
	"user_id" CHAR(20) NOT NULL, -- 用户ID
	"is_default" BOOLEAN NOT NULL DEFAULT FALSE, -- 是否默认地址
	"consignee" VARCHAR(20) NOT NULL, -- 收货人
	"phone" VARCHAR(20) NOT NULL, -- 电话
	"address" VARCHAR(255) NOT NULL, -- 地址
	"province" VARCHAR(10) NOT NULL, -- 省份
	"city" VARCHAR(100) NOT NULL DEFAULT '', -- 城市
	"county" VARCHAR(100) NOT NULL DEFAULT '', -- 县
	"post_code" CHAR(6) NOT NULL DEFAULT '', -- 邮编
	"is_del" BOOLEAN NOT NULL DEFAULT FALSE, -- 是否删除
	"dateline" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP -- 添加时间
);

CREATE TYPE "order_status" AS ENUM( -- 订单状态
	'Unspecified', -- 未知
	'PendingPay', -- 待支付（已创建订单）
	'UserCancel', -- 用户取消
	'AdminCancel',  -- 管理员取消
	'TimeoutCancel', -- 超时
	'Paied', -- 已支付（等待发货）
	'Delivering', -- 运输中（已发货）
	'Delivered', -- 已送达（等待用户确认）
	'UserConfirmedDone', -- 完成（用户确认收货）
	'AutoConfirmedDone' -- 完成（自动确认收货）
);
CREATE TABLE IF NOT EXISTS "orders" ( -- 订单
	"id" CHAR(20) PRIMARY KEY,
	"user_id" CHAR(20) NOT NULL, -- 用户ID
	"cart_id" CHAR(20) NOT NULL, -- 购物车ID
	"sn" VARCHAR(64) NOT NULL, -- 编号
	"status" order_status NOT NULL DEFAULT 'Unspecified', -- 状态
	"amount" u32 NOT NULL, -- 金额
	"total_num" u32 NOT NULL, -- 总数
	"freight" u32 NOT NULL DEFAULT 0, -- 运费
	"address" JSONB NOT NULL, -- 收货地址
	"delivery_id" VARCHAR(64) NOT NULL DEFAULT '', -- 快递单号
	"dateline" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP, -- 下单时间
	"cancel_until_dateline" TIMESTAMPTZ NOT NULL, -- 自动取消订单的时间
	"confirm_until_dateline" TIMESTAMPTZ NOT NULL, -- 自动确认订单的时间
	"is_del" BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS "order_goods" ( -- 订单商品
	"id" CHAR(20) PRIMARY KEY,
	"order_id" CHAR(20) NOT NULL, -- 订单ID
	"goods_id" CHAR(20) NOT NULL, -- 商品ID
	"goods_snapshot" JSONB NOT NULL, -- 商品快照
	"num" u32 NOT NULL, -- 购买数量
	"price" u32 NOT NULL -- 购买单价
);
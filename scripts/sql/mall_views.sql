CREATE OR REPLACE VIEW v_category_brands AS -- 分类与品牌
SELECT
	c.id, c."name", parent, "path", "level", c.dateline, c.is_del
	, b.id AS brand_id, b."name" AS brand_name, b.logo AS brand_logo, b.is_del AS brand_is_del, b.dateline AS brand_dateline
FROM categoies AS c
INNER JOIN
	category_brands AS  cb 
ON 
	cb.category_id = c.id 
INNER JOIN 
	brands AS b 
ON
	cb.brand_id  = b.id 
;
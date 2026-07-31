use super::*;
use std::collections::HashMap;

#[test]
fn test_sort_direction_sql() {
    assert_eq!(SortDirection::Asc.sql(), "ASC");
    assert_eq!(SortDirection::Desc.sql(), "DESC");
}

#[test]
fn test_sort_direction_parse() {
    assert_eq!("asc".parse::<SortDirection>().unwrap(), SortDirection::Asc);
    assert_eq!(
        "desc".parse::<SortDirection>().unwrap(),
        SortDirection::Desc
    );
    assert_eq!("ASC".parse::<SortDirection>().unwrap(), SortDirection::Asc);
    assert_eq!(
        "DESC".parse::<SortDirection>().unwrap(),
        SortDirection::Desc
    );
    assert!("invalid".parse::<SortDirection>().is_err());
}

#[test]
fn test_parse_sort_string() {
    let sorts = parse_sort_string("title:asc,createdAt:desc");
    assert_eq!(sorts.len(), 2);
    assert_eq!(sorts[0].field, "title");
    assert_eq!(sorts[0].direction, SortDirection::Asc);
    assert_eq!(sorts[1].field, "createdAt");
    assert_eq!(sorts[1].direction, SortDirection::Desc);
}

#[test]
fn test_parse_sort_string_no_direction() {
    let sorts = parse_sort_string("title,name");
    assert_eq!(sorts.len(), 2);
    assert_eq!(sorts[0].direction, SortDirection::Asc);
    assert_eq!(sorts[1].direction, SortDirection::Asc);
}

#[test]
fn test_parse_sort_string_empty() {
    let sorts = parse_sort_string("");
    assert!(sorts.is_empty());
}

#[test]
fn test_filter_operator_sql() {
    assert_eq!(FilterOperator::Eq.sql(), "=");
    assert_eq!(FilterOperator::Ne.sql(), "!=");
    assert_eq!(FilterOperator::Gt.sql(), ">");
    assert_eq!(FilterOperator::Gte.sql(), ">=");
    assert_eq!(FilterOperator::Lt.sql(), "<");
    assert_eq!(FilterOperator::Lte.sql(), "<=");
    assert_eq!(FilterOperator::Like.sql(), "ILIKE");
}

#[test]
fn test_filter_operator_parse() {
    assert_eq!("eq".parse::<FilterOperator>().unwrap(), FilterOperator::Eq);
    assert_eq!(
        "gte".parse::<FilterOperator>().unwrap(),
        FilterOperator::Gte
    );
    assert_eq!(
        "like".parse::<FilterOperator>().unwrap(),
        FilterOperator::Like
    );
}

#[test]
fn test_sql_order_by() {
    let params = QueryParams {
        sort: parse_sort_string("title:asc,weight:desc"),
        ..Default::default()
    };

    let mut field_map = HashMap::new();
    field_map.insert("title", "m.title");
    field_map.insert("weight", "m.weight");

    let clause = params.sql_order_by(&field_map);
    assert_eq!(clause, "m.title ASC, m.weight DESC");
}

#[test]
fn test_sql_order_by_unknown_field() {
    let params = QueryParams {
        sort: parse_sort_string("title:asc,unknown:desc"),
        ..Default::default()
    };

    let mut field_map = HashMap::new();
    field_map.insert("title", "m.title");

    let clause = params.sql_order_by(&field_map);
    assert_eq!(clause, "m.title ASC"); // unknown field is ignored
}

#[test]
fn test_where_builder() {
    let filters = vec![
        FilterField::eq("pathwayId", "abc123"),
        FilterField::new("weight", FilterOperator::Gte, "10"),
    ];

    let mut field_map = HashMap::new();
    field_map.insert("pathwayId", "m.pathway_id");
    field_map.insert("weight", "m.weight");

    let mut builder = WhereBuilder::new(1);
    for filter in &filters {
        builder.add_filter(filter, &field_map);
    }

    let (clause, values) = builder.build();
    assert_eq!(clause, "m.pathway_id = $1 AND m.weight >= $2");
    assert_eq!(values, vec!["abc123", "10"]);
}

#[test]
fn test_where_builder_with_where() {
    let filters = vec![FilterField::eq("status", "active")];

    let mut field_map = HashMap::new();
    field_map.insert("status", "m.status");

    let mut builder = WhereBuilder::new(1);
    for filter in &filters {
        builder.add_filter(filter, &field_map);
    }

    let (clause, _) = builder.build_with_where();
    assert_eq!(clause, "WHERE m.status = $1");
}

#[test]
fn test_where_builder_empty() {
    let builder = WhereBuilder::new(1);
    let (clause, values) = builder.build_with_where();
    assert_eq!(clause, "");
    assert!(values.is_empty());
}

#[test]
fn test_query_params_serde_urlencoded() {
    // Test that QueryParams can be deserialized from URL query string format
    let query_string = "sort=title%3Aasc%2Cweight%3Adesc";
    let params: QueryParams = serde_urlencoded::from_str(query_string).unwrap();

    assert_eq!(params.sort.len(), 2);
    assert_eq!(params.sort[0].field, "title");
    assert_eq!(params.sort[0].direction, SortDirection::Asc);
    assert_eq!(params.sort[1].field, "weight");
    assert_eq!(params.sort[1].direction, SortDirection::Desc);
}

#[test]
fn test_query_params_serde_urlencoded_single_sort() {
    let query_string = "sort=weight%3Adesc";
    let params: QueryParams = serde_urlencoded::from_str(query_string).unwrap();

    assert_eq!(params.sort.len(), 1);
    assert_eq!(params.sort[0].field, "weight");
    assert_eq!(params.sort[0].direction, SortDirection::Desc);
}

#[test]
fn test_query_params_serde_urlencoded_empty() {
    let query_string = "";
    let params: QueryParams = serde_urlencoded::from_str(query_string).unwrap();

    assert!(params.sort.is_empty());
}

#[test]
fn test_query_params_serde_urlencoded_with_filter() {
    let query_string = "sort=title%3Aasc&filter%5BpathwayId%5D=abc123";
    let params: QueryParams = serde_urlencoded::from_str(query_string).unwrap();

    assert_eq!(params.sort.len(), 1);
    assert_eq!(params.sort[0].field, "title");

    let filters = params.filter_fields();
    assert_eq!(filters.len(), 1);
    assert_eq!(filters[0].field, "pathwayId");
    assert_eq!(filters[0].value, "abc123");
}

#[test]
fn test_query_params_serde_urlencoded_with_like_filter() {
    // filter[name][like]=%test%
    let query_string = "filter%5Bname%5D%5Blike%5D=%25test%25";
    let params: QueryParams = serde_urlencoded::from_str(query_string).unwrap();

    let filters = params.filter_fields();
    assert_eq!(filters.len(), 1);
    assert_eq!(filters[0].field, "name");
    assert_eq!(filters[0].operator, FilterOperator::Like);
    assert_eq!(filters[0].value, "%test%");
}

#[test]
fn test_field_mapping_map() {
    let mapping = FieldMapping::new()
        .map("title", "m.title")
        .map("isLive", "m.is_live");

    let sort = mapping.sort_map();
    assert_eq!(sort.get("title"), Some(&"m.title"));
    assert_eq!(sort.get("isLive"), Some(&"m.is_live"));

    let filter = mapping.filter_map();
    assert_eq!(filter.get("title"), Some(&"m.title"));
    assert_eq!(filter.get("isLive"), Some(&"m.is_live"));
}

#[test]
fn test_field_mapping_sort_only() {
    let mapping = FieldMapping::new()
        .map("title", "m.title")
        .sort_only("weight", "m.weight");

    let sort = mapping.sort_map();
    assert_eq!(sort.get("weight"), Some(&"m.weight"));

    let filter = mapping.filter_map();
    assert_eq!(filter.get("weight"), None);
    assert_eq!(filter.get("title"), Some(&"m.title"));
}

#[test]
fn test_field_mapping_filter_only() {
    let mapping = FieldMapping::new()
        .map("title", "m.title")
        .filter_only("pathwayId", "m.pathway_id");

    let sort = mapping.sort_map();
    assert_eq!(sort.get("pathwayId"), None);

    let filter = mapping.filter_map();
    assert_eq!(filter.get("pathwayId"), Some(&"m.pathway_id"));
}

#[test]
fn test_field_mapping_get() {
    let mapping = FieldMapping::new()
        .map("title", "m.title")
        .sort_only("weight", "m.weight")
        .filter_only("pathwayId", "m.pathway_id");

    assert_eq!(mapping.get_sort("title"), Some("m.title"));
    assert_eq!(mapping.get_sort("weight"), Some("m.weight"));
    assert_eq!(mapping.get_sort("pathwayId"), None);

    assert_eq!(mapping.get_filter("title"), Some("m.title"));
    assert_eq!(mapping.get_filter("weight"), None);
    assert_eq!(mapping.get_filter("pathwayId"), Some("m.pathway_id"));
}

#[test]
fn test_field_mapping_macro() {
    let mapping = crate::field_mapping! {
        "title" => "m.title",
        "slug" => "m.slug",
    };

    assert_eq!(mapping.get_sort("title"), Some("m.title"));
    assert_eq!(mapping.get_filter("slug"), Some("m.slug"));
}

#[test]
fn test_list_query_params_unified_extraction() {
    let params: ListQueryParams = serde_urlencoded::from_str(
        "page=2&limit=25&sort=title:asc&filter[status]=active&variant=live&search=hello%20world",
    )
    .unwrap();

    assert_eq!(params.pagination.page, 2);
    assert_eq!(params.pagination.limit, 25);
    assert_eq!(params.offset_i64(), 25);
    assert_eq!(params.variant.as_deref(), Some("live"));
    assert_eq!(params.search_term(), Some("hello world"));

    let filters = params.filter_fields();
    assert_eq!(filters.len(), 1);
    assert_eq!(filters[0].field, "status");
}

#[test]
fn test_list_query_params_defaults_and_clamp() {
    let params: ListQueryParams = serde_urlencoded::from_str("").unwrap();
    assert_eq!(params.pagination.page, 1);
    assert_eq!(params.pagination.limit, 20);
    assert!(params.variant.is_none());
    assert!(params.search_term().is_none());

    let params: ListQueryParams = serde_urlencoded::from_str("limit=500").unwrap();
    assert_eq!(params.clamped().pagination.limit, 100);
}

#[test]
fn test_list_query_params_search_term_trims() {
    let params: ListQueryParams = serde_urlencoded::from_str("search=%20%20").unwrap();
    assert!(params.search_term().is_none());
}

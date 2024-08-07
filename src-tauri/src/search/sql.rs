use itertools::Itertools;

use super::{ParsedKeyword, ParsedMeta, ParsedTag, Search};

pub fn build_search_sql_stmt(search: Search, page: i32, page_size: i32) -> Result<String, String> {
    let Search {
        tags,
        keywords,
        meta,
    } = search;
    let stmt = if tags.is_empty() {
        build_main_stem("sticker", &keywords, &meta, false)
    } else {
        Ok(format!(
            "WITH taged_sticker AS ({}) {}",
            build_sub_query_tages(&tags).unwrap(),
            build_main_stem("taged_sticker", &keywords, &meta, false)?
        ))
    };
    stmt.map(|mut stmt: String| {
        stmt.push_str(&format!(" LIMIT {} OFFSET {}", page_size, page * page_size));
        stmt.push(';');
        stmt
    })
}

pub fn build_count_sql_stmt(search: Search) -> Result<String, String>{ 
    let Search {
        tags,
        keywords,
        meta,
    } = search;
    let stmt = if tags.is_empty() {
        build_main_stem("sticker", &keywords, &meta, true)
    } else {
        Ok(format!(
            "WITH taged_sticker AS ({}) {}",
            build_sub_query_tages(&tags).unwrap(),
            build_main_stem("taged_sticker", &keywords, &meta, true)?
        ))
    };
    stmt
}

fn build_main_stem(
    src_table: &str,
    keywords: &Vec<ParsedKeyword>,
    meta: &Vec<ParsedMeta>,
    count_number: bool
) -> Result<String, String> {
    let mut sources = Vec::new();
    let mut cond = Vec::new();
    let mut order_by = None;
    let mut order_rev = false;

    sources.push(format!("{} as inp", src_table));

    for kwd in keywords {
        cond.push(format!(
            "inp.name {} '%{}%'",
            if kwd.not { "NOT LIKE" } else { "LIKE" },
            kwd.value
        ))
    }

    for m in meta.iter().unique_by(|m| m.key) {
        match m.key {
            super::MetaKey::DateStart => {
                cond.push(format!("inp.modify_date <= {}", m.value));
            }
            super::MetaKey::DateEnd => {
                cond.push(format!("inp.modify_date >= {}", m.value));
            }
            super::MetaKey::Package => {
                sources.push(String::from("JOIN package on inp.package = package.id"));
                cond.push(format!("package.name = '{}'", m.value));
            }
            super::MetaKey::Ty => {
                cond.push(format!("inp.type = '{}'", m.value));
            }
            super::MetaKey::Sort => match m.value.as_ref() {
                "name" => {
                    order_by = Some("inp.name");
                }
                "create" => {
                    order_by = Some("inp.create_date");
                }
                "modify" => {
                    order_by = Some("inp.modify_date");
                }
                _ => {
                    Err(format!(
                        "{} is not sortable. it must be one of 'name', 'create' or 'modify'",
                        m.value
                    ))?;
                }
            },
            super::MetaKey::Order => match m.value.as_ref() {
                "asc" => {
                    order_rev = false;
                }
                "desc" => {
                    order_rev = true;
                }
                _ => {
                    Err(format!("Order must be 'asc' or 'desc'"))?;
                }
            },
        };
    }

    Ok(format!(
        "SELECT {} FROM {}{} {}",
        if count_number {
            "COUNT(inp.id)"
        }else{
            "inp.*"
        },
        sources
            .into_iter()
            .reduce(|pre, acc| format!("{} {}", pre, acc))
            .unwrap(),
        cond.into_iter()
            .reduce(|pre, acc| format!("{} AND {}", pre, acc))
            .map(|c| format!(" WHERE {}", c))
            .unwrap_or(String::new()),
        order_by
            .map(|v| format!("ORDER BY {} {}", v, if order_rev { "DESC" } else { "ASC" }))
            .unwrap_or_else(|| String::new()),
    ))
}

/*
Result is like:

```sql
SELECT sticker.*
FROM sticker
         JOIN sticker_tag ON sticker.id = sticker_tag.sticker
WHERE sticker_tag.tag IN (SELECT id as tag FROM tag WHERE (tag.namespace = 'female' AND tag.value = 'lolicon'))
GROUP BY sticker.id
HAVING COUNT(DISTINCT sticker_tag.tag) = 1
   AND sticker.id NOT IN (SELECT sticker.id
                         FROM sticker
                                  JOIN sticker_tag ON sticker.id = sticker_tag.sticker
                                  JOIN tag ON sticker_tag.tag = tag.id
                         WHERE (tag.namespace = 'misc' AND tag.value = 'nsfw'));

```
 */
fn build_sub_query_tages(tags: &Vec<ParsedTag>) -> Option<String> {
    if tags.is_empty() {
        return None;
    }
    let tags_has_all = tags.iter().filter(|tag| !tag.not).collect::<Vec<_>>();
    let tags_none_of = tags.iter().filter(|tag| tag.not).collect::<Vec<_>>();

    let mut select_sticker =
        "SELECT sticker.* FROM sticker JOIN sticker_tag ON sticker.id = sticker_tag.sticker WHERE"
            .to_string();

    let has_all_cond = if !tags_has_all.is_empty() {
        Some(format!(
            " sticker_tag.tag IN (SELECT id as tag FROM tag WHERE {}) GROUP BY sticker.id HAVING COUNT(DISTINCT sticker_tag.tag) = {}",
            tags_has_all
                .iter()
                .map(|t| {
                    format!(
                        "(tag.namespace = '{}' AND tag.value = '{}')",
                        t.namespace, t.value
                    )
                })
                .reduce(|pre, acc| format!("{} OR {}", pre, acc))
                .unwrap(),
            tags_has_all.len()
        ))
    } else {
        None
    };

    let none_of_cond = if !tags_none_of.is_empty() {
        Some(format!(
            " sticker.id NOT IN (SELECT sticker.id FROM sticker JOIN sticker_tag ON sticker.id = sticker_tag.sticker JOIN tag ON sticker_tag.tag = tag.id WHERE {})",
            tags_none_of
                .iter()
                .map(|t| {
                    format!(
                        "(tag.namespace = '{}' AND tag.value = '{}')",
                        t.namespace, t.value
                    )
                })
                .reduce(|pre, acc| format!("{} OR {}", pre, acc))
                .unwrap()
        ))
    } else {
        None
    };
    if let Some(first_cond) = has_all_cond {
        select_sticker.push_str(&first_cond);
        if let Some(second_cond) = none_of_cond {
            select_sticker.push_str(" AND");
            select_sticker.push_str(&second_cond);
        }
    } else if let Some(second_cond) = none_of_cond {
        select_sticker.push_str(&second_cond);
        select_sticker.push_str(" GROUP BY sticker.id");
    }

    Some(select_sticker)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::search::{MetaKey, ParsedTag, Search};

    #[test]
    fn test_full_serach() {
        let s = Search {
            tags: vec![
                ParsedTag {
                    value: String::from("lolicon"),
                    namespace: String::from("female"),
                    not: false,
                },
                ParsedTag {
                    not: true,
                    value: String::from("guro"),
                    namespace: String::from("female"),
                },
            ],
            keywords: vec![
                ParsedKeyword {
                    not: false,
                    value: "刷".to_string(),
                },
                ParsedKeyword {
                    not: true,
                    value: "朱重八".to_string(),
                },
            ],
            meta: vec![ParsedMeta {
                key: MetaKey::Package,
                not: false,
                value: "Inbox".to_string(),
            }],
        };
        println!("{}", build_search_sql_stmt(s, 0, 30).unwrap());
    }

    #[test]
    fn test_stem() {
        assert_eq!(
            build_main_stem(
                "sticker",
                &vec![
                    ParsedKeyword {
                        not: false,
                        value: "刷".to_string()
                    },
                    ParsedKeyword {
                        not: true,
                        value: "朱重八".to_string()
                    }
                ],
                &Vec::new(),
                false
            ).unwrap(),
            String::from("SELECT inp.* FROM sticker as inp JOIN package on inp.package = package.id WHERE package.name = '穗穗'")
        );
        assert_eq!(
            build_main_stem(
                "sticker",
                &vec![
                ],
                &vec![
                    ParsedMeta{
                        key: MetaKey::Package,
                        not: false,
                        value: "穗穗".to_string()
                    }
                ],
                false
            ).unwrap(),
            String::from("SELECT inp.* FROM sticker as inp WHERE inp.name LIKE '%刷%' AND inp.name NOT LIKE '%朱重八%'")
        );
    }

    #[test]
    fn test_tag() {
        assert_eq!(build_sub_query_tages(&vec![]), None);
        assert_eq!(
            build_sub_query_tages(&vec![
                ParsedTag {
                    value: String::from("lolicon"),
                    namespace: String::from("female"),
                    not: false,
                },
            ]),
            Some("SELECT sticker.* FROM sticker JOIN sticker_tag ON sticker.id = sticker_tag.sticker WHERE sticker_tag.tag IN (SELECT id as tag FROM tag WHERE (tag.namespace = 'female' AND tag.value = 'lolicon')) GROUP BY sticker.id HAVING COUNT(DISTINCT sticker_tag.tag) = 1".to_string())
        );
        assert_eq!(
            build_sub_query_tages(&vec![
                ParsedTag {
                    not: true,
                    value: String::from("nsfw"),
                    namespace: String::from("misc")
                }
            ]),
            Some("SELECT sticker.* FROM sticker JOIN sticker_tag ON sticker.id = sticker_tag.sticker WHERE sticker_tag.tag IN (SELECT id as tag FROM tag WHERE (tag.namespace = 'female' AND tag.value = 'lolicon')) GROUP BY sticker.id HAVING COUNT(DISTINCT sticker_tag.tag) = 1 AND sticker.id NOT IN (SELECT sticker.id FROM sticker JOIN sticker_tag ON sticker.id = sticker_tag.sticker JOIN tag ON sticker_tag.tag = tag.id WHERE (tag.namespace = 'misc' AND tag.value = 'nsfw'))".to_string())
        );
        assert_eq!(
            build_sub_query_tages(&vec![
                ParsedTag {
                    value: String::from("lolicon"),
                    namespace: String::from("female"),
                    not: false,
                },
                ParsedTag {
                    not: true,
                    value: String::from("nsfw"),
                    namespace: String::from("misc")
                }
            ]),
            Some("SELECT sticker.* FROM sticker JOIN sticker_tag ON sticker.id = sticker_tag.sticker WHERE sticker_tag.tag IN (SELECT id as tag FROM tag WHERE (tag.namespace = 'female' AND tag.value = 'lolicon')) GROUP BY sticker.id HAVING COUNT(DISTINCT sticker_tag.tag) = 1 AND sticker.id NOT IN (SELECT sticker.id FROM sticker JOIN sticker_tag ON sticker.id = sticker_tag.sticker JOIN tag ON sticker_tag.tag = tag.id WHERE (tag.namespace = 'misc' AND tag.value = 'nsfw'))".to_string())
        );
    }
}

-- Add migration script here
CREATE VIEW instance AS
 WITH domain_counts(domain, accounts_count) AS (
         SELECT accounts.domain,
            count(*) AS accounts_count
           FROM accounts
          WHERE (accounts.domain IS NOT NULL)
          GROUP BY accounts.domain
        )
SELECT domain_counts.domain,domain_counts.accounts_count
FROM domain_counts
UNION
 SELECT domain_blocks.domain,
    COALESCE(domain_counts.accounts_count, 0) AS accounts_count
   FROM (domain_blocks
     LEFT JOIN domain_counts ON ((domain_counts.domain = domain_blocks.domain)))
UNION
 SELECT domain_allows.domain,
    COALESCE(domain_counts.accounts_count, 0) AS accounts_count
   FROM (domain_allows
     LEFT JOIN domain_counts ON ((domain_counts.domain = domain_allows.domain)))
;

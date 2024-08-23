CREATE VIEW instance AS
 WITH domain_counts(domain, account_count) AS (
         SELECT account.domain,
            count(*) AS account_count
           FROM account
          WHERE (account.domain IS NOT NULL)
          GROUP BY account.domain
        )
SELECT domain_counts.domain,domain_counts.account_count
FROM domain_counts
UNION
 SELECT domain_block.domain,
    COALESCE(domain_counts.account_count, 0) AS account_count
   FROM (domain_block
     LEFT JOIN domain_counts ON ((domain_counts.domain = domain_block.domain)))
UNION
 SELECT domain_allow.domain,
    COALESCE(domain_counts.account_count, 0) AS account_count
   FROM (domain_allow
     LEFT JOIN domain_counts ON ((domain_counts.domain = domain_allow.domain)))
;

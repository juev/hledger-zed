; Plain transactions with date and payee
(plain_xact
  (date) @context
  (payee) @name) @item

; Periodic transactions with interval
(periodic_xact
  (interval) @name) @item

; Automated transactions with query
(automated_xact
  (query) @name) @item

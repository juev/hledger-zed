; Directives
[
  "account"
  "alias"
  "assert"
  "check"
  "commodity"
  "comment"
  "def"
  "default"
  "end"
  "eval"
  "format"
  "nomarket"
  "note"
  "payee"
  "test"
  "A"
  "Y"
  "N"
  "D"
  "C"
  "P"
] @keyword

"include" @keyword.import

; Dates
(date) @string.special
(effective_date) @string.special
(time) @string.special
(interval) @string.special

; Transaction status markers (* and !)
(status) @attribute

; Transaction code (in parentheses)
(code) @string.special

; Transaction payee/description
(payee) @string

; Notes
(note) @comment

; Account names
(account) @variable

; Amounts and numbers
(quantity) @number
(negative_quantity) @number

; Commodities (currency symbols and codes)
(commodity) @type

; Comments
(comment) @comment
(block_comment) @comment

; Balance assertions
(balance_assertion) @operator

; Cost/price specifications
(price) @operator
(lot_price) @operator

; Periodic transaction marker
"~" @keyword.repeat

; Automated transaction marker
"=" @keyword

; Operators
"@" @operator
"@@" @operator

; Punctuation
[
  "("
  ")"
  "["
  "]"
] @punctuation.bracket

";" @punctuation.delimiter
"#" @punctuation.delimiter

; Error nodes
(ERROR) @error

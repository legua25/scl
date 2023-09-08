// scl.g4
grammar scl;

// Parser
//// Document
document: (entry+ | lit)? EOF;
entry:
    ident ':' lit                                                                                                       # key_value_entry |
    ident '[' ((lit ',')* lit ','?)? ']'                                                                                # list_entry |
    ident '{' ((entry ',')* entry ','?)? '}'                                                                            # struct_entry
;

//// Literal values
lit:
    Bool                                                                                                                # bool_lit |
    Int                                                                                                                 # int_lit |
    Float                                                                                                               # float_lit |
    Decimal                                                                                                             # decimal_lit |
    (Str | Text)                                                                                                        # string_lit |
    Timestamp                                                                                                           # timestamp_lit |
    Binary                                                                                                              # binary_lit |
    '[' ((lit ',')* lit ','?)? ']'                                                                                      # list_lit |
    '{' ((entry ',')* entry ','?)? '}'                                                                                  # struct_lit
;

//// Identifiers
ident: id = raw_ident Str?;
raw_ident: Ident | Str;

// Lexer
//// Literals
Bool: 'true' | 'false';
Int: Integer;
Float: Fractional;
Decimal: Fractional 'd';
Str: '"' (~["\\\n\r] | Escape)* '"';
Text: '"""' (~[\\] | '\\' .)*? '"""';
Timestamp: Date? 'T' Time | Date;
Binary: '{{' Whitespace* ((MimeType ':')? Blob | MimeType ':')? Whitespace* '}}';

fragment Hex: [0-9a-fA-F];
fragment Integer: '0' | '-'? [1-9] ('_'? [0-9])*;
fragment Fractional: '-'? Integer '.' [0-9] ('_'? [0-9])* Exp? | Integer Exp;
fragment Exp: 'e' [+-]? Integer;

fragment Escape: '\\' (["\\nrt] | AsciiEscape | Utf8Escape);
fragment AsciiEscape: 'x' Hex Hex;
fragment Utf8Escape: 'u' Hex Hex? Hex? Hex? Hex? Hex?;

fragment Date: Year '-' Month '-' Day;
fragment Time: Hour ':' Minutes (':' Seconds Offset?)?;
fragment Year: '000' [1-9] | '00' [1-9] [0-9] | '0' [1-9] [0-9] [0-9] | [1-9] [0-9] [0-9] [0-9];
fragment Month: '0' [1-9] | '1' [0-2];
fragment Day: '0' [1-9] | [12] [0-9] | '3' [0-1];
fragment Hour: [01] [0-9] | '2' [0-3];
fragment Minutes: [0-5] [0-9];
fragment Seconds: [0-5] [0-9] ('.' [0-9]+)?;
fragment Offset: 'Z' | [+-] Hour ':' Minutes;

fragment MimeType: MimePart '/' MimePart;
fragment MimePart: [A-Za-z] [A-Za-z0-9-+.]* | '*';
fragment Blob: BlobWord+ BlobPad?;
fragment BlobWord: B64 B64 B64 B64;
fragment BlobPad: B64 B64 B64 '=' | B64 B64 '==';
fragment B64: [0-9a-zA-Z+/];

//// Identifier
Ident: [\p{L}_] [\p{L}\p{N}\p{Mn}\p{Mc}_]*;

//// Whitespace and comments
Whitespace: [ \n\r\t\u000C] -> skip;
Comment: '#' ~[\r\n]* -> channel(HIDDEN);

Unknown: .;

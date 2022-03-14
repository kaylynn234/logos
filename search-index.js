var searchIndex = JSON.parse('{\
"logos":{"doc":"Logos","t":[13,16,16,4,3,8,8,24,3,13,16,6,16,10,11,10,11,11,11,11,11,11,11,11,11,11,0,11,11,11,11,11,11,11,0,11,11,11,12,11,11,11,11,11,11,11,11,11,11,10,11,0,10,11,11,11,11,11,11,11,11,11,11,11,11,5,11,11,11,0,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,8,13,13,13,4,13,11,11,11,11,10,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,8,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,10,11,3,3,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,8,18,16,8,11,10,10,11,10,10,10,10,10],"n":["Accept","Error","Extras","Filter","Lexer","LexerExt","Logos","Logos","Skip","Skip","Source","Span","Token","as_lexer","as_lexer","as_lexer_mut","as_lexer_mut","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","boxed","boxed","bump","callback","clone","clone_into","construct","construct","construct","construct","construct","error","error","extras","extras","extras","extras_mut","extras_mut","fmt","from","from","from","into","into","into","into_iter","into_lexer","into_lexer","iter","lex","lexer","lexer_with_extras","lookahead","lookahead","map_with_lexer","map_with_lexer","morph","new","next","remainder","remainder","remainder","skip","slice","slice","slice","source","source","source","source","span","span","span","spanned","to_owned","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","with_extras","0","CallbackResult","Construct","Emit","Error","Output","Skip","borrow","borrow_mut","clone","clone_into","construct","construct","construct","eq","fmt","from","hash","into","ne","to_owned","try_from","try_into","type_id","0","0","0","Error","UnknownToken","borrow","borrow_mut","clone","clone_into","cmp","construct","default","eq","fmt","fmt","from","hash","into","partial_cmp","to_owned","to_string","try_from","try_into","type_id","unknown_token","unknown_token","BoxedLexer","Lookahead","MapWithLexer","as_lexer","as_lexer","as_lexer","as_lexer_mut","as_lexer_mut","as_lexer_mut","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","construct","construct","construct","count","fold","from","from","from","into","into","into","into_iter","into_iter","into_iter","into_lexer","into_lexer","into_lexer","last","next","next","next","next_if","next_if_eq","nth","peek","peek_mut","size_hint","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","Chunk","SIZE","Slice","Source","find_boundary","from_ptr","is_boundary","is_empty","len","read","read_unchecked","slice","slice_unchecked"],"q":["logos","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","logos::Filter","logos::callback","","","","","","","","","","","","","","","","","","","","","","","logos::callback::Output","","","logos::error","","","","","","","","","","","","","","","","","","","","","","","logos::iter","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","logos::source","","","","","","","","","","","",""],"d":["Construct and emit a variant containing a value of type <code>C</code>.","The type used to report errors during lexing.","The “extras” type, used to add state to a lexer.","A type that can be used within callbacks to either produce …","A <code>Lexer</code> allows you to read through a source (a type …","Extension methods for types that contain a Lexer.","Types that can be lexed using Lexer.","","Used within callbacks to instruct the lexer to skip a …","Skip this token match.","The source type that tokens are lexed from.","A byte range in the source.","The token type that the Lexer uses.","Get a reference to the underlying Lexer.","","Get a mutable reference to the underlying Lexer.","","","","","","","","Box the lexer, returning a type-erased BoxedLexer.","Box the lexer, returning a type-erased BoxedLexer.","Bump the current span by <code>n</code> bytes.","Types and traits used within lexer callbacks.","","","","","","","","Types and traits for error handling in Logos.","Create a new error value representing a generic “unknown …","Get a reference to the lexer’s extras. This is a …","Get a reference to the lexer’s extras. This is a …","The “extras” associated with <code>Token</code>.","Get a mutable reference to the lexer’s extras. This is a …","Get a mutable reference to the lexer’s extras. This is a …","","","","","","","","","Consume <code>self</code>, and return the underlying Lexer.","","Tools for working with lexers as iterators.","The heart of Logos.","Create a new Lexer for this token type.","Create a new Lexer for this token type, using the provided …","Wrap the Lexer in an Iterator that can use the peek and …","Wrap the Lexer in an Iterator that can use the peek and …","Wrap the lexer in an Iterator that maps each token to …","Wrap the lexer in an Iterator that maps each token to …","Turn this lexer into a lexer for a new token type.","Create a new <code>Lexer</code>.","","A slice containing the remaining source. This is similar …","A slice containing the remaining source. This is similar …","A slice containing the remaining source. This is similar …","A predefined callback that unconditionally skips a token …","A slice containing the current token. The return type of …","A slice containing the current token. The return type of …","A slice containing the current token. The return type of …","Traits for reading from different input sources.","The source that tokens are being read from. The return …","The source that tokens are being read from. The return …","The source that tokens are being read from. The return …","The source position of the current token. This is …","The source position of the current token. This is …","The source position of the current token.","Wrap the lexer in an Iterator that pairs tokens with their …","","","","","","","","","","","Create a new <code>Lexer</code> with the provided extras.","","Types that can be returned from lexer callbacks.","Construct a variant containing a value of type <code>C</code>","Emit a token of type <code>T</code>","Emit an error of type <code>E</code>","Represents actions the lexer can take.","Skip this token","","","","","Construct an Output value using <code>self</code>, instructing the …","","","","","","","","","","","","","","","","A trait for representing errors that occur during lexing.","The primary error case when lexing, and the default error …","","","","","","","","","","","","","","","","","","","","Creates an error value representing an unknown token. This …","","A boxed and type-erased lexer.","An iterator with a <code>peek()</code> method that can look into the …","An iterator that maps each value to another, making use of …","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Advance the lexer and return the next token, but only if a …","Advance the lexer and return the next token, but only if …","","Returns a reference to the next token, without advancing …","Returns a mutable reference to the next token, without …","","","","","","","","","","","A fixed, statically sized chunk of data that can be read …","The size of the chunk, in bytes.","A slice of this <code>Source</code>","Types the <code>Lexer</code> can read from.","Find the closest valid index for this <code>Source</code>, starting at …","Create a chunk from a raw pointer.","Check if <code>index</code> is valid for this <code>Source</code>. Namely, ensure …","Whether the source is empty.","The length of the source, in bytes.","Read a chunk of bytes into an array. Returns <code>None</code> when …","Read a chunk of bytes into an array, without performing …","Return the slice of input corresponding to <code>range</code>, or <code>None</code> …","Return the slice of input corresponding to <code>range</code>, <strong>without </strong>…"],"i":[1,2,2,0,0,0,0,0,0,1,2,0,3,3,4,3,4,4,5,1,4,5,1,3,3,4,0,4,4,4,5,5,1,1,0,4,3,3,4,3,3,4,4,5,1,4,5,1,4,3,4,0,2,2,2,3,3,3,3,4,4,4,3,3,4,0,3,3,4,0,3,3,4,3,3,4,4,4,4,5,1,4,5,1,4,5,1,4,6,0,7,7,7,0,7,7,7,7,7,8,7,7,7,7,7,7,7,7,7,7,7,7,9,10,11,0,0,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,13,12,0,0,0,14,15,16,14,15,16,14,15,16,14,15,16,14,15,16,16,16,14,15,16,14,15,16,14,15,16,14,15,16,16,14,15,16,16,16,16,16,16,16,14,15,16,14,15,16,14,15,16,0,17,18,0,18,17,18,18,18,18,18,18,18],"f":[null,null,null,null,null,null,null,null,null,null,null,null,null,[[],["lexer",3]],[[],["lexer",3]],[[],["lexer",3]],[[],["lexer",3]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["boxedlexer",3]],[[],["boxedlexer",3]],[[["usize",15]]],null,[[]],[[]],[[["lexer",3]],["output",4]],[[["lexer",3]],["output",4]],[[["lexer",3]],["output",4]],[[["lexer",3]],["output",4]],[[["lexer",3]],["output",4]],null,[[]],[[]],[[]],null,[[]],[[]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["lexer",3]],[[],["lexer",3]],null,[[["lexer",3]]],[[],["lexer",3]],[[],["lexer",3]],[[],["lookahead",3]],[[],["lookahead",3]],[[],["mapwithlexer",3]],[[],["mapwithlexer",3]],[[],["lexer",3]],[[]],[[],["option",4]],[[]],[[]],[[]],[[["lexer",3]],["skip",3]],[[]],[[]],[[]],null,[[]],[[]],[[]],[[],["span",6]],[[],["span",6]],[[],["span",6]],[[],["mapwithlexer",3]],[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[]],null,null,null,null,null,null,null,[[]],[[]],[[],["output",4]],[[]],[[["lexer",3]],["output",4]],[[["lexer",3]],["output",4]],[[["lexer",3]],["output",4]],[[["output",4]],["bool",15]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[["output",4]],["bool",15]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null,null,null,null,[[]],[[]],[[],["unknowntoken",3]],[[]],[[["unknowntoken",3]],["ordering",4]],[[["lexer",3]],["output",4]],[[],["unknowntoken",3]],[[["unknowntoken",3]],["bool",15]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[["unknowntoken",3]],["option",4,[["ordering",4]]]],[[]],[[],["string",3]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[["lexer",3]]],[[["lexer",3]]],null,null,null,[[],["lexer",3]],[[],["lexer",3]],[[],["lexer",3]],[[],["lexer",3]],[[],["lexer",3]],[[],["lexer",3]],[[]],[[]],[[]],[[]],[[]],[[]],[[["lexer",3]],["output",4]],[[["lexer",3]],["output",4]],[[["lexer",3]],["output",4]],[[],["usize",15]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["lexer",3]],[[],["lexer",3]],[[],["lexer",3]],[[],["option",4]],[[],["option",4]],[[],["option",4]],[[],["option",4]],[[],["option",4]],[[],["option",4]],[[["usize",15]],["option",4]],[[],["option",4]],[[],["option",4]],[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],null,null,null,null,[[["usize",15]],["usize",15]],[[]],[[["usize",15]],["bool",15]],[[],["bool",15]],[[],["usize",15]],[[["usize",15]],["option",4]],[[["usize",15]]],[[["range",3,[["usize",15]]]],["option",4]],[[["range",3,[["usize",15]]]]]],"p":[[4,"Filter"],[8,"Logos"],[8,"LexerExt"],[3,"Lexer"],[3,"Skip"],[13,"Accept"],[4,"Output"],[8,"CallbackResult"],[13,"Construct"],[13,"Emit"],[13,"Error"],[3,"UnknownToken"],[8,"Error"],[3,"BoxedLexer"],[3,"MapWithLexer"],[3,"Lookahead"],[8,"Chunk"],[8,"Source"]]},\
"logos_derive":{"doc":"Logos","t":[24],"n":["Logos"],"q":["logos_derive"],"d":[""],"i":[0],"f":[null],"p":[]},\
"tests":{"doc":"","t":[5],"n":["assert_lex"],"q":["tests"],"d":[""],"i":[0],"f":[[[]]],"p":[]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};
var N=null,E="",T="t",U="u",searchIndex={};
var R=["string","result","try_from","try_into","borrow_mut","borrow","type_id","typeid","to_owned","clone_into","basictype","wrapper","socketaddr","duration","instant","systemtime","ipv4addr","ipv6addr","socketaddrv4","socketaddrv6","messagewrapper","message","formatter","serialize","serializer","MessageWrapper","BasicType"];
searchIndex["errors"]={"doc":E,"i":[[3,"Wrapper","errors","Wrapper is a very strange type which exists for…",N,N],[12,"0",E,E,0,N],[3,"Message",E,"Message type effectively doesn't exist, it just wraps…",N,N],[3,R[25],E,"MessageWrapper is a strange type It works much like the…",N,N],[12,"0",E,E,1,N],[3,"Err",E,"Err is a heavy, but complete custom error type system.",N,N],[4,R[26],E,"BasicType is used as a \"relatively\" efficient way to store…",N,N],[13,"Bool",E,E,2,N],[13,"I8",E,E,2,N],[13,"I16",E,E,2,N],[13,"I32",E,E,2,N],[13,"I64",E,E,2,N],[13,"I128",E,E,2,N],[13,"U8",E,E,2,N],[13,"U16",E,E,2,N],[13,"U32",E,E,2,N],[13,"U64",E,E,2,N],[13,"U128",E,E,2,N],[13,"F32",E,E,2,N],[13,"F64",E,E,2,N],[13,"StaticStr",E,E,2,N],[13,"IP",E,E,2,N],[13,"Socket",E,E,2,N],[13,"Dur",E,E,2,N],[13,"Inst",E,E,2,N],[13,"SysTime",E,E,2,N],[13,"String",E,E,2,N],[13,"Debug",E,E,2,N],[13,"IOError",E,E,2,N],[11,"err",E,"error can work with most error/message formats",3,[[["self"],["a"],["c"]],["self"]]],[11,"note",E,"appends kv data to an existing error",3,[[["a"],["str"]],["self"]]],[11,"to_json",E,"serializes the data into a compact JSON representation",3,[[["self"]],[[R[1],[R[0]]],[R[0]]]]],[11,"to_json_pretty",E,"serializes the data into a human readable JSON…",3,[[["self"]],[[R[1],[R[0]]],[R[0]]]]],[11,"into",E,E,0,[[],[U]]],[11,"from",E,E,0,[[[T]],[T]]],[11,R[2],E,E,0,[[[U]],[R[1]]]],[11,R[3],E,E,0,[[],[R[1]]]],[11,R[4],E,E,0,[[["self"]],[T]]],[11,R[5],E,E,0,[[["self"]],[T]]],[11,R[6],E,E,0,[[["self"]],[R[7]]]],[11,"into",E,E,4,[[],[U]]],[11,"from",E,E,4,[[[T]],[T]]],[11,R[2],E,E,4,[[[U]],[R[1]]]],[11,R[3],E,E,4,[[],[R[1]]]],[11,R[4],E,E,4,[[["self"]],[T]]],[11,R[5],E,E,4,[[["self"]],[T]]],[11,R[6],E,E,4,[[["self"]],[R[7]]]],[11,"into",E,E,1,[[],[U]]],[11,"from",E,E,1,[[[T]],[T]]],[11,R[2],E,E,1,[[[U]],[R[1]]]],[11,R[3],E,E,1,[[],[R[1]]]],[11,R[4],E,E,1,[[["self"]],[T]]],[11,R[5],E,E,1,[[["self"]],[T]]],[11,R[6],E,E,1,[[["self"]],[R[7]]]],[11,R[8],E,E,3,[[["self"]],[T]]],[11,R[9],E,E,3,[[["self"],[T]]]],[11,"into",E,E,3,[[],[U]]],[11,"from",E,E,3,[[[T]],[T]]],[11,"to_string",E,E,3,[[["self"]],[R[0]]]],[11,R[2],E,E,3,[[[U]],[R[1]]]],[11,R[3],E,E,3,[[],[R[1]]]],[11,R[4],E,E,3,[[["self"]],[T]]],[11,R[5],E,E,3,[[["self"]],[T]]],[11,R[6],E,E,3,[[["self"]],[R[7]]]],[11,R[8],E,E,2,[[["self"]],[T]]],[11,R[9],E,E,2,[[["self"],[T]]]],[11,"into",E,E,2,[[],[U]]],[11,"from",E,E,2,[[[T]],[T]]],[11,R[2],E,E,2,[[[U]],[R[1]]]],[11,R[3],E,E,2,[[],[R[1]]]],[11,R[4],E,E,2,[[["self"]],[T]]],[11,R[5],E,E,2,[[["self"]],[T]]],[11,R[6],E,E,2,[[["self"]],[R[7]]]],[11,"into",E,E,4,[[],[R[0]]]],[11,"default",E,E,3,[[],["err"]]],[11,"clone",E,E,2,[[["self"]],[R[10]]]],[11,"clone",E,E,3,[[["self"]],["err"]]],[11,"from",E,E,0,[[["d"]],["self"]]],[11,"from",E,E,0,[[["arguments"]],["self"]]],[11,"from",E,E,2,[[[R[11]]],["self"]]],[11,"from",E,E,0,[[[R[0]]],["self"]]],[11,"from",E,E,2,[[["arc",["str"]],[R[11],["arc"]]],[R[10]]]],[11,"from",E,E,0,[[["bool"]],["self"]]],[11,"from",E,E,0,[[["bool"]],["self"]]],[11,"from",E,E,2,[[[R[11],["bool"]],["bool"]],["self"]]],[11,"from",E,E,0,[[["i8"]],["self"]]],[11,"from",E,E,0,[[["i8"]],["self"]]],[11,"from",E,E,2,[[["i8"],[R[11],["i8"]]],["self"]]],[11,"from",E,E,0,[[["i16"]],["self"]]],[11,"from",E,E,0,[[["i16"]],["self"]]],[11,"from",E,E,2,[[[R[11],["i16"]],["i16"]],["self"]]],[11,"from",E,E,0,[[["i32"]],["self"]]],[11,"from",E,E,0,[[["i32"]],["self"]]],[11,"from",E,E,2,[[[R[11],["i32"]],["i32"]],["self"]]],[11,"from",E,E,0,[[["i64"]],["self"]]],[11,"from",E,E,0,[[["i64"]],["self"]]],[11,"from",E,E,2,[[["i64"],[R[11],["i64"]]],["self"]]],[11,"from",E,E,0,[[["i128"]],["self"]]],[11,"from",E,E,0,[[["i128"]],["self"]]],[11,"from",E,E,2,[[["i128"],[R[11],["i128"]]],["self"]]],[11,"from",E,E,0,[[["u8"]],["self"]]],[11,"from",E,E,0,[[["u8"]],["self"]]],[11,"from",E,E,2,[[[R[11],["u8"]],["u8"]],["self"]]],[11,"from",E,E,0,[[["u16"]],["self"]]],[11,"from",E,E,0,[[["u16"]],["self"]]],[11,"from",E,E,2,[[["u16"],[R[11],["u16"]]],["self"]]],[11,"from",E,E,0,[[["u32"]],["self"]]],[11,"from",E,E,0,[[["u32"]],["self"]]],[11,"from",E,E,2,[[[R[11],["u32"]],["u32"]],["self"]]],[11,"from",E,E,0,[[["u64"]],["self"]]],[11,"from",E,E,0,[[["u64"]],["self"]]],[11,"from",E,E,2,[[[R[11],["u64"]],["u64"]],["self"]]],[11,"from",E,E,0,[[["u128"]],["self"]]],[11,"from",E,E,0,[[["u128"]],["self"]]],[11,"from",E,E,2,[[[R[11],["u128"]],["u128"]],["self"]]],[11,"from",E,E,0,[[["f32"]],["self"]]],[11,"from",E,E,0,[[["f32"]],["self"]]],[11,"from",E,E,2,[[["f32"],[R[11],["f32"]]],["self"]]],[11,"from",E,E,0,[[["f64"]],["self"]]],[11,"from",E,E,0,[[["f64"]],["self"]]],[11,"from",E,E,2,[[[R[11],["f64"]],["f64"]],["self"]]],[11,"from",E,E,0,[[["ipaddr"]],["self"]]],[11,"from",E,E,0,[[["ipaddr"]],["self"]]],[11,"from",E,E,2,[[[R[11],["ipaddr"]],["ipaddr"]],["self"]]],[11,"from",E,E,0,[[[R[12]]],["self"]]],[11,"from",E,E,0,[[[R[12]]],["self"]]],[11,"from",E,E,2,[[[R[11],[R[12]]],[R[12]]],["self"]]],[11,"from",E,E,0,[[[R[13]]],["self"]]],[11,"from",E,E,0,[[[R[13]]],["self"]]],[11,"from",E,E,2,[[[R[11],[R[13]]],[R[13]]],["self"]]],[11,"from",E,E,0,[[[R[14]]],["self"]]],[11,"from",E,E,0,[[[R[14]]],["self"]]],[11,"from",E,E,2,[[[R[14]],[R[11],[R[14]]]],["self"]]],[11,"from",E,E,0,[[[R[15]]],["self"]]],[11,"from",E,E,0,[[[R[15]]],["self"]]],[11,"from",E,E,2,[[[R[11],[R[15]]],[R[15]]],["self"]]],[11,"from",E,E,0,[[["error"]],["self"]]],[11,"from",E,E,2,[[["error"],[R[11],["error"]]],["self"]]],[11,"from",E,E,0,[[["str"]],["self"]]],[11,"from",E,E,0,[[["str"]],["self"]]],[11,"from",E,E,2,[[[R[11],["str"]],["str"]],["self"]]],[11,"from",E,E,0,[[[R[16]]],["self"]]],[11,"from",E,E,0,[[[R[16]]],["self"]]],[11,"from",E,E,2,[[[R[11],[R[16]]],[R[16]]],["self"]]],[11,"from",E,E,0,[[[R[17]]],["self"]]],[11,"from",E,E,0,[[[R[17]]],["self"]]],[11,"from",E,E,2,[[[R[17]],[R[11],[R[17]]]],["self"]]],[11,"from",E,E,0,[[[R[18]]],["self"]]],[11,"from",E,E,0,[[[R[18]]],["self"]]],[11,"from",E,E,2,[[[R[11],[R[18]]],[R[18]]],["self"]]],[11,"from",E,E,0,[[[R[19]]],["self"]]],[11,"from",E,E,0,[[[R[19]]],["self"]]],[11,"from",E,E,2,[[[R[19]],[R[11],[R[19]]]],["self"]]],[11,"from",E,E,1,[[["str"]],["self"]]],[11,"from",E,E,1,[[["cow"]],["self"]]],[11,"from",E,E,1,[[[R[0]]],["self"]]],[11,"from",E,E,1,[[["str"]],["self"]]],[11,"from",E,E,1,[[["cow"]],["self"]]],[11,"from",E,E,1,[[[R[0]]],["self"]]],[11,"from",E,E,1,[[["e"]],["self"]]],[11,"from",E,E,4,[[[R[20],["box"]],["box",["error"]]],[R[21]]]],[11,"from",E,E,1,[[["s"]],["self"]]],[11,"from",E,E,4,[[[R[20],["box"]],["box",["asref"]]],[R[21]]]],[11,"from",E,E,1,[[["s"]],["self"]]],[11,"from",E,E,4,[[[R[20],["box"]],["box",["deref"]]],[R[21]]]],[11,"from",E,E,1,[[["s"]],["self"]]],[11,"from",E,E,4,[[["box",["deref"]],[R[20],["box"]]],[R[21]]]],[11,"from",E,E,1,[[["s"]],["self"]]],[11,"from",E,E,4,[[[R[20],["box"]],["box",["asref"]]],[R[21]]]],[11,"from",E,E,1,[[["d"]],["self"]]],[11,"from",E,E,4,[[[R[20],["box"]],["box",["debug"]]],[R[21]]]],[11,"from",E,E,1,[[["d"]],["self"]]],[11,"from",E,E,4,[[[R[20],["box"]],["box",["display"]]],[R[21]]]],[11,"from",E,E,1,[[["box"]],["self"]]],[11,"from",E,E,4,[[[R[20],["box"]],["box"]],[R[21]]]],[11,"from",E,E,1,[[["cow"]],["self"]]],[11,"from",E,E,4,[[["cow"],[R[20],["cow"]]],[R[21]]]],[11,"from",E,E,1,[[["str"],["cow",["str"]]],["self"]]],[11,"from",E,E,4,[[[R[20],["cow"]],["cow",["str"]]],[R[21]]]],[11,"from",E,E,1,[[[R[0]]],["self"]]],[11,"from",E,E,4,[[[R[20],[R[0]]],[R[0]]],[R[21]]]],[11,"from",E,E,1,[[["str"],["box",["str"]]],["self"]]],[11,"from",E,E,4,[[[R[20],["box"]],["box",["str"]]],[R[21]]]],[11,"from",E,E,1,[[[R[0]]],["self"]]],[11,"from",E,E,4,[[[R[0]],[R[20],[R[0]]]],[R[21]]]],[11,"from",E,E,4,[[["str"],["box",["str"]]],[R[21]]]],[11,"from",E,E,4,[[[R[0]]],[R[21]]]],[11,"fmt",E,E,2,[[["self"],[R[22]]],[R[1]]]],[11,"fmt",E,E,3,[[["self"],[R[22]]],[R[1]]]],[11,"fmt",E,E,3,[[["self"],[R[22]]],[R[1]]]],[11,"description",E,E,3,[[["self"]],["str"]]],[11,R[23],E,E,2,[[["self"],[R[24]]],[R[1]]]],[11,R[23],E,E,3,[[["self"],[R[24]]],[R[1]]]]],"p":[[3,"Wrapper"],[3,R[25]],[4,R[26]],[3,"Err"],[3,"Message"]]};
initSearch(searchIndex);addSearchOptions(searchIndex);
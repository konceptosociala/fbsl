rule DEFAULT;
rule DEBUG;

input my_input: u32;

/*
 *
 * there can be some multiline comments
 *
 */
 
function main() 
{
    let x: u32;
}

function x(a: u32, _b: bool){
    let x: u32 = 2;
    x = 2; // and also some single comments
    _b = a;
    x = a + _b * 2 - 3;
}

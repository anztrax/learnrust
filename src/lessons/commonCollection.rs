#[derive(Debug)]
enum SpreadSheetCell{
    Int(i32),
    Float(f64),
    Text(String)
}

pub fn tryCommonCollection(){
    let v = vec![1,2,3];
    println!("vector value : {:#?}", v);

    let mut vector1 = Vec::new();
    vector1.push(5);
    vector1.push(6);
    vector1.push(7);
    println!("vector1 value: {:#?}",vector1);

    let vector_item2_value = v[2];
    println!("vector item : {}", vector_item2_value);
    println!("vector value : {:#?}", v);


    let v = vec![1, 2, 3,4,5,6,7];
    let v_index = 2;
    let v_reference = &v[v_index];
    match v.get(v_index){
        Some(_) => {
            println!("Reachable element at index: {}", v_index);
        },
        None => {
            println!("Unreachable element at index: {}", v_index);
        }
    }


    //this can't be done because
    let mut v = vec![1,2,3,4,5];
    let first = &v[0].clone();
    v.push(6);
    println!("v value : {:?}, first value : {}", v, first);


    //iterating over the value in a vector, using & for de-referencing data
    let v = vec![100, 32, 57];
    for i in &v{
        println!("{}", i);
    }
    println!("v : {:?}",v);


    //mutable is for editing the value
    let mut v = vec![100, 32, 27, 50];
    for i in &mut v{    //i is immutable, so we need de-referencing i
        *i += 10;
    }
    println!("v : {:?}",v);


    //save enum
    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12)
    ];
    println!("row data : {:?}", row);
}
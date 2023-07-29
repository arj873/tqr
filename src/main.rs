fn main() {
    println!("Hello, world!");
    let mut arr: [[bool; 21]; 21] = [[false; 21]; 21];
    arr = addPositioningThings(arr);
    // println!("{:?}",arr);
    for i in arr.iter(){
        for x in i.iter(){
                print!("{}", if x == &true {"\u{2588}\u{2588}"} else {"  "} );
        }
        println!("")
    }
}

fn addPositioningThings(mut arr: [[bool; 21]; 21]) -> [[bool; 21]; 21]{
    arr = placePositioning(arr, 0, 0);
    arr = placePositioning(arr, arr.len()-7, 0);    
    arr = placePositioning(arr, 0, arr.len()-7);


    arr
}

fn placePositioning(mut arr: [[bool; 21]; 21],offsetX :usize,offsetY :usize) -> [[bool; 21]; 21]{
    //outside top
    arr[0+offsetX][0+offsetY] = true;
    arr[1+offsetX][0+offsetY] = true;
    arr[2+offsetX][0+offsetY] = true;
    arr[3+offsetX][0+offsetY] = true;
    arr[4+offsetX][0+offsetY] = true;
    arr[5+offsetX][0+offsetY] = true;
    arr[6+offsetX][0+offsetY] = true;

    //outside  right
    arr[6+offsetX][0+offsetY] = true;
    arr[6+offsetX][1+offsetY] = true;
    arr[6+offsetX][2+offsetY] = true;
    arr[6+offsetX][3+offsetY] = true;
    arr[6+offsetX][4+offsetY] = true;
    arr[6+offsetX][5+offsetY] = true;
    arr[6+offsetX][6+offsetY] = true;

    //outside  left
    arr[0+offsetX][0+offsetY] = true;
    arr[0+offsetX][1+offsetY] = true;
    arr[0+offsetX][2+offsetY] = true;
    arr[0+offsetX][3+offsetY] = true;
    arr[0+offsetX][4+offsetY] = true;
    arr[0+offsetX][5+offsetY] = true;
    arr[0+offsetX][6+offsetY] = true;

    //outside bottom
    arr[0+offsetX][6+offsetY] = true;
    arr[1+offsetX][6+offsetY] = true;
    arr[2+offsetX][6+offsetY] = true;
    arr[3+offsetX][6+offsetY] = true;
    arr[4+offsetX][6+offsetY] = true;
    arr[5+offsetX][6+offsetY] = true;
    arr[6+offsetX][6+offsetY] = true;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    //inside top
    arr[2+offsetX][2+offsetY] = true;
    arr[3+offsetX][2+offsetY] = true;
    arr[4+offsetX][2+offsetY] = true;  

    //inside  right
    arr[4+offsetX][2+offsetY] = true;
    arr[4+offsetX][3+offsetY] = true;
    arr[4+offsetX][4+offsetY] = true;

    //inside  left
    arr[2+offsetX][2+offsetY] = true;
    arr[2+offsetX][3+offsetY] = true;
    arr[2+offsetX][4+offsetY] = true;

    //inside bottom
    arr[2+offsetX][4+offsetY] = true;
    arr[3+offsetX][4+offsetY] = true;
    arr[4+offsetX][4+offsetY] = true;

    //inside middle
    arr[3+offsetX][3+offsetY] = true;


    arr


}
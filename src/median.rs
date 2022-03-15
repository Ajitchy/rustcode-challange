
pub fn median( mut a:vec<f32>) -> Option<f32>{
    //Case1. When list of number is empty

    if a.is_empty(){
        return None;
    }
    a.sort_by( |x:&f32,y:f32| x.partial_cmp(&y).unwrap());
    //Todo sort
    let n_elements = a.len();
    let middle = n_elements/2;
    //case2. when list of number is odd
    //case3. when list of number is even
    let med = if n_elements%2 ==0 {
        //even
        (a[middle-1]+ a[middle]) / 2.0
    }else {
        //odd
        a[middle]
    };
    Some(med)
}
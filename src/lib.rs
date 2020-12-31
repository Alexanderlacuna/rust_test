#[cfg(test)]
mod test;
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self,rectangle:&Rectangle)->bool{

        return self.width>rectangle.width && self.height>rectangle.height
    }
}

fn test_panicking(val_to_unwrap:&str)->i32{
    let results=val_to_unwrap.trim().parse::<i32>().unwrap();
    results

}


#[cfg(test)]
mod tests {


    use super::{
        Rectangle,
        test_panicking
    };
    #[test]
    // #[ignore = "no reason for ignoring"]
    fn it_works(){
        assert_ne!(3,4)
    }
    #[test]
    fn test_can_holds(){
        let rectangle=Rectangle {width:12,height:5};
        let  container=Rectangle {width:6,height:5};

        assert_eq!(rectangle.can_hold(&container),false)
    }

    fn normal_function(){
        println!("hello there");
    }

    // should panic
    #[test]
    #[should_panic]
    fn test_parse_string(){
        
            test_panicking("hello");
    }
}
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }

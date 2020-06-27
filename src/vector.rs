
pub fn vector_demo(){

   let mut  vec1 =Vec::new();
   vec1.push(23);
   vec1.push(3);
   println!("vec1: {:?}",vec1);
   let   vec2 = vec![1,2,3];
   let mut vec3 =vec!["a","b","c"];
   println!("vec2: {:?}",vec2);
   vec3.pop();
   let   index3 =vec3[1];
   println!("index=1 :{:?}",index3);
   println!("vec2: {:?}",vec3);
   
   for i in &vec3{
       println!("v={:?}",i);
   }

}
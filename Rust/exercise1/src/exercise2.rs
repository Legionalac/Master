fn get_min_max_value(list : &Vec<i32>) -> (&i32,&i32){
   let mut min: &i32;
   let mut max: &i32;
   min = &list[0];
   max = &list[1];

   for value in list{
      if value > max {
         max = value;
      }

      if value < min {
         min = value;
      }

   }

   return (min, max);
}

fn main()
{
   let mut list: Vec<i32> = Vec::new();
   list.push(10);
   list.push(20);
   list.push(30);
   list.push(40);
   list.push(50);

   let (min, max) = get_min_max_value(&list);
   println!("List : {:?}", list);
   println!("Min : {}", min);
   println!("Max : {}", max);
}

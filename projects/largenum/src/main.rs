fn main() {
   let numlist = vec![12, 30, 50, 60, 78, 23, 12, 43, 634, 65, 765, 99];

   let mut larget = &numlist[0];

   for num in &numlist {
      if num > larget {
        larget = num;
      }
   }
   println!("{}", larget);
}

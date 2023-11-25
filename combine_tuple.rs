struct Project<T,U>(T,U);

trait Tuple{
    type ProductList;
}

trait ProductList{
    type Tuple;
}

impl<T> Tuple for (T,){
    type ProductList = Project<T,()>;
}

impl<T> ProductList for Project<T,()>{
    type Tuple = (T,);
}

impl<T,U> Tuple for (T,U){
    type ProductList = Project<T,Project<U,()>>;
} 

impl<T,U> ProductList for Project<T,Project<U,()>>{
    type Tuple = (T,U,);
}
impl<T,U,V> ProductList for Project<T,Project<U,Project<V,()>>>{
    type Tuple = (T,U,V,);
}

trait Combin<U>{
    type Output;
}

impl<U> Combin<U> for (){
    type Output = U;
} 

impl<X,Y,U> Combin<U> for Project<X,Y>
where Y:Combin<U>{
    type Output = Project<X,Y::Output>;
} 

type TupleToProduct<T> = <T as Tuple>::ProductList;
type ProductToTuple<T> = <T as ProductList>::Tuple;

fn main(){
    type R = <TupleToProduct<(i32,u8)> as Combin<TupleToProduct<(String,)>>>::Output;
    type RTuple = ProductToTuple<R>;
    let c = RTuple::default();  // (i32, u8,String)
}

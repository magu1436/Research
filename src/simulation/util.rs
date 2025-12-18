use arrayfire::*;

pub fn create_array_replaced_elem<T>(arr: &Array<T>, row: usize, col: usize, value: T) -> Array<T>
where T: Default + Copy + HasAfEnum {
    let mut host = vec![T::default(); arr.elements()];
    arr.host(&mut host);
    
    let n_rows = arr.dims().get()[0] as usize;
    let idx = row + col * n_rows;
    host[idx] = value;

    Array::new(&host, arr.dims())
}

pub fn ref_in_array<T>(arr: &Array<T>, row: usize, col: usize) -> T
where T: Default + Copy + HasAfEnum {
    let n_rows = arr.dims().get()[0] as usize;
    let mut host = vec![T::default(); arr.elements()];
    arr.host(&mut host);

    let idx: usize = row + col * n_rows;
    host[idx]
}
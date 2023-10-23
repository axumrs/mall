/// 笛卡尔乘积
pub struct Cartesian<'a, T> {
    data: &'a Vec<Vec<T>>,
}

impl<'a, T> Cartesian<'a, T> {
    pub fn new(data: &'a Vec<Vec<T>>) -> Self {
        Self { data }
    }
}

impl<'a, T: Clone> Cartesian<'a, T> {
    fn partial(&self, v: Vec<Vec<T>>, i: Vec<T>) -> Vec<Vec<T>> {
        v.into_iter()
            .flat_map(|xs| {
                i.iter()
                    .cloned()
                    .map(|y| {
                        let mut vec = xs.clone();
                        vec.push(y);
                        vec
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    /// 生成笛卡尔乘积
    pub fn product(&self) -> Vec<Vec<T>> {
        if let Some((first, els)) = self.data.split_first() {
            let init = first
                .iter()
                .cloned()
                .map(|s| vec![s])
                .collect::<Vec<Vec<T>>>();

            return els
                .iter()
                .cloned()
                .fold(init, |vec, list| self.partial(vec, list));
        }
        vec![]
    }
}

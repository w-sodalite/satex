pub trait Apply: Sized {
    ///
    /// 通过移动所有权，修改自身的可变性并在闭包中使用，然后返回所有权。主要用来简化闭包的处理，但是会带来可读性的下降。
    ///
    /// # Arguments
    ///
    /// * `f`: 处理函数
    ///
    /// returns: Self
    ///
    /// # Examples
    ///
    /// - 正常方式
    ///
    /// ```
    /// let items = vec!["A".to_string(),"B".to_string()];
    /// let items = items.into_iter().map(|mut item| {
    ///     item.push('1');
    ///     item
    /// }).collect::<Vec<_>>();
    /// assert_eq!(items,vec!["A1".to_string(),"B1".to_string()]);
    /// ```
    ///
    /// - 使用Apply
    ///
    /// ```
    /// use satex_core::apply::Apply;
    /// let items = vec!["A".to_string(),"B".to_string()];
    /// let items = items.into_iter().map(|item|item.apply(|value|value.push('1')) ).collect::<Vec<_>>();
    /// assert_eq!(items,vec!["A1".to_string(),"B1".to_string()]);
    /// ```
    ///
    fn apply<R, F: FnOnce(&mut Self) -> R>(self, f: F) -> Self;
}

impl<T> Apply for T {
    fn apply<R, F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut Self) -> R,
    {
        f(&mut self);
        self
    }
}

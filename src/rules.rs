pub struct Rule<'rule> {
    pub(crate) desc: &'rule str,
    pub(crate) value: i32,
    pub(crate) name: &'rule str,
    pub(crate) rtype: &'rule str,
    pub(crate) max: i32,
    pub(crate) min: i32
}
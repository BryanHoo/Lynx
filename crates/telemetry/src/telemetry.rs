/// 保留遥测调用点的语法和类型检查，但不在运行时求值或分配事件数据。
#[macro_export]
macro_rules! event {
    ($name:expr) => {{
        if false {
            let _ = &$name;
        }
    }};
    ($name:expr, $($key:ident $(= $value:expr)?),+ $(,)?) => {{
        if false {
            let _ = &$name;
            $(let _ = &$crate::event_property!($key $(= $value)?);)+
        }
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! event_property {
    ($key:ident) => {
        $key
    };
    ($key:ident = $value:expr) => {
        $value
    };
}

/// 接受并立即丢弃旧遥测发送端，使现有初始化代码自然结束接收任务。
pub fn init<T>(_sender: T) {}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    #[test]
    fn disabled_event_does_not_evaluate_arguments() {
        let evaluated = Cell::new(false);

        crate::event!(
            "Disabled Event",
            value = {
                evaluated.set(true);
                1
            }
        );

        assert!(!evaluated.get());
    }
}

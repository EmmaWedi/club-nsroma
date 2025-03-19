// Create a dedicated macros module
pub mod macros {
    #[macro_export]
    macro_rules! apply_update_wrap {
        ($model:ident, $dto:ident, $($model_field:ident : $dto_field:ident $(=> $wrap:tt)?),+) => {
            $(
                if let Some(value) = $dto.$dto_field {
                    $model.$model_field = Set($( $wrap )?(value));
                }
            )+
        };
    }

    #[macro_export]
    macro_rules! apply_update_no_wrap {
        ($model:ident, $dto:ident, $($model_field:ident: $dto_field:ident),+) => {
            $(
                if let Some(value) = $dto.$dto_field {
                    $model.$model_field = Set(value);
                }
            )+
        };
    }
}



#[macro_export]
macro_rules! into_active_model {
    (
        $struct_name:ident, $active_model:ident, 
        { $(mandatory: $mandatory_field:ident),* $(,)? }, 
        { $(optional: $optional_field:ident),* $(,)? }
    ) => {
        impl IntoActiveModel for $struct_name {
            type Model = $active_model;

            fn into_active_model(self) -> Self::Model {
                let mut model = <Self::Model as sea_orm::ActiveModelTrait>::default();
                $(
                    model.$mandatory_field = ActiveValue::Set(self.$mandatory_field);
                )*
                $(
                    if let Some(value) = self.$optional_field {
                        model.$optional_field = ActiveValue::Set(value);
                    }
                )*
                model
            }
        }
    }
}

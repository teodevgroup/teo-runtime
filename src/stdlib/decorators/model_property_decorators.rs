use crate::namespace::Namespace;

pub(in crate::stdlib) fn load_model_property_decorators(namespace: &mut Namespace) {

    namespace.define_model_property_decorator("getter", |arguments, property| {
        Ok(())
    });

    // /// @name Getter
    // /// Define a property with getter
    // declare unique model property decorator getter(pipeline?: Pipeline<Self, ThisFieldType>)
    //
    // /// @name Setter
    // /// Define a property with setter
    // declare unique model property decorator setter(pipeline?: Pipeline<Self, Ignored>)
    //
    // /// @name Cache
    // /// Define a cached property, a cached property is saved into the database
    // declare unique model property decorator cached
    //
    // /// @name Dependencies
    // /// Define dependencies for a cached property
    // declare unique model property decorator deps(deps?: ModelScalarFieldsWithoutVirtuals<Self> | ModelScalarFieldsWithoutVirtuals<Self>[])
    //
    // /// @name Index
    // /// Define index for this cached property
    // declare unique model property decorator index(sort: Sort?, length: Int?, map: String?)
    //
    // /// @name Unique
    // /// Define unique index for this cached property
    // declare unique model property decorator unique(sort: Sort?, length: Int?, map: String?)

}
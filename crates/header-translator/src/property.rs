use clang::{Entity, EntityKind, EntityVisitResult, Nullability, ObjCAttributes};

use crate::availability::Availability;
use crate::config::MethodData;
use crate::method::{MemoryManagement, Method, Qualifier};
use crate::rust_type::Ty;
use crate::unexposed_macro::UnexposedMacro;

#[derive(Debug, Clone)]
pub struct PartialProperty<'tu> {
    pub entity: Entity<'tu>,
    pub name: String,
    pub getter_name: String,
    pub setter_name: Option<String>,
    pub is_class: bool,
    pub attributes: Option<ObjCAttributes>,
}

impl PartialProperty<'_> {
    pub fn parse(
        self,
        getter_data: MethodData,
        setter_data: Option<MethodData>,
    ) -> (Option<Method>, Option<Method>) {
        let Self {
            entity,
            name,
            getter_name,
            setter_name,
            is_class,
            attributes,
        } = self;

        let availability = Availability::parse(
            entity
                .get_platform_availability()
                .expect("method availability"),
        );

        // `@property(copy)` for some reason returns nonnull?
        //
        // Swift signifies that they use forced unwrapping here, perhaps
        // because they know that it can fail (e.g. in OOM situations), but
        // is very unlikely to?
        let default_nullability = if attributes.map(|a| a.copy).unwrap_or(false) {
            Nullability::NonNull
        } else {
            Nullability::Unspecified
        };

        let mut memory_management = MemoryManagement::Normal;

        entity.visit_children(|entity, _parent| {
            match entity.get_kind() {
                EntityKind::ObjCClassRef
                | EntityKind::ObjCProtocolRef
                | EntityKind::TypeRef
                | EntityKind::ParmDecl => {
                    // Ignore
                }
                EntityKind::ObjCReturnsInnerPointer => {
                    if memory_management != MemoryManagement::Normal {
                        panic!("got unexpected ObjCReturnsInnerPointer")
                    }
                    memory_management = MemoryManagement::ReturnsInnerPointer;
                }
                EntityKind::ObjCInstanceMethodDecl => {
                    println!("WARNING: method in property {name:?}");
                }
                EntityKind::IbOutletAttr => {
                    // TODO: What is this?
                }
                EntityKind::UnexposedAttr => {
                    if let Some(macro_) = UnexposedMacro::parse(&entity) {
                        println!("WARNING: macro in property {name:?}: {macro_:?}");
                    }
                }
                _ => panic!("Unknown property child: {entity:?}, {name:?}"),
            };
            EntityVisitResult::Continue
        });

        let qualifier = entity.get_objc_qualifiers().map(Qualifier::parse);
        assert!(qualifier.is_none(), "properties do not support qualifiers");

        let getter = if !getter_data.skipped {
            let ty = Ty::parse_property_return(
                entity.get_type().expect("property type"),
                default_nullability,
            );

            Some(Method {
                selector: getter_name.clone(),
                fn_name: getter_name,
                availability: availability.clone(),
                is_class,
                is_optional_protocol: entity.is_objc_optional(),
                memory_management,
                designated_initializer: false,
                arguments: Vec::new(),
                result_type: ty,
                safe: !getter_data.unsafe_,
                mutating: getter_data.mutating,
            })
        } else {
            None
        };

        let setter = if let Some(setter_name) = setter_name {
            let setter_data = setter_data.expect("setter_data must be present if setter_name was");
            if !setter_data.skipped {
                let ty = Ty::parse_property(
                    entity.get_type().expect("property type"),
                    Nullability::Unspecified,
                );

                Some(Method {
                    selector: setter_name.clone() + ":",
                    fn_name: setter_name,
                    availability: availability.clone(),
                    is_class,
                    is_optional_protocol: entity.is_objc_optional(),
                    memory_management,
                    designated_initializer: false,
                    arguments: vec![(name, None, ty)],
                    result_type: Ty::VOID_RESULT,
                    safe: !setter_data.unsafe_,
                    mutating: setter_data.mutating,
                })
            } else {
                None
            }
        } else {
            None
        };

        (getter, setter)
    }
}

# Grost Object Definition

## Overview

Groto objects support:

- Schema metadata for documentation and identification
- Custom flavors for custom encode/decode flavor system
- Multiple object variants (partial, reference, partial-reference)
- Type-safe field labeling with comprehensive type system
- Default value handling with type-specific strategies
- Generic parameter customization for advanced use cases
- Selector and indexer generation for data access patterns
- Flexible encoding/decoding configuration per field
- Cross-variant conversion between different object types

## Container Attributes

### Core Attributes

| Attribute | Description | Type | Default | Optional |
|:---------:|-------------|------|---------|:--------:|
| `default`   | Specify the default fn to create an object (If not provided, the code will not generate `Default` implementation for the object) | `String \| Closure` | N/A | Yes |
| `schema`    | Defines metadata including name and documentation for the object | [`Schema`](#container-schema-attribute) | See [`Schema`](#container-schema-attribute) | Yes |
| `generic`   | Customizes the names of generic parameters used in generated code | [`Generic`](#generic-attribute) | See [`Generic`](#generic-attribute) | Yes |
| `flavor`  | Registers a custom flavor for specialized serialization instead of the default flavor | [`Flavor`](#flavor-attribute) | See [`Flavor`](#flavor-attribute) | Yes |
| `partial` | Configures generation of partial objects | [`Partial`](#partial-attribute) | See [`Partial`](#partial-attribute) | Yes |
| `partial_ref` | Configures generation of partial reference objects | [`PartialRef`](#partial-ref-attribute) | See [`PartialRef`](#partial-ref-attribute) | Yes |
| `ref` | Configures generation of reference objects | [`Ref`](#ref-attribute) | See [`Ref`](#ref-attribute) | Yes |
| `selector` | Configures generation of selector of the objects | [`Selector`](#selector-attribute) | See [`Selector`](#selector-attribute) | Yes |
| `indexer` | Configures generation of indexer of the objects | [`Indexer`](#indexer-attribute) | See [`Indexer`](#indexer-attribute) | Yes |

### Global Default Value Handling

These attributes provide global defaults during decoding or converting that can be overridden by field-level and type-specific configurations.

| Attribute | Description | Type | Default | Optional |
|:---------:|-------------|------|---------|:--------:|
| `or_default` | Global setting: provide default values for missing fields during deserialization | `bool` | `false` | Yes |
| `or_default_scalar` | Provide defaults for missing scalar fields (numbers, booleans, and etc) | `bool` | `false` | Yes |
| `or_default_string` | Provide defaults for missing string fields | `bool` | `false` | Yes |
| `or_default_bytes` | Provide defaults for missing byte array fields | `bool` | `false` | Yes |
| `or_default_object` | Provide defaults for missing nested object fields | `bool` | `false` | Yes |
| `or_default_enum` | Provide defaults for missing enum fields | `bool` | `false` | Yes |
| `or_default_union` | Provide defaults for missing union fields | `bool` | `false` | Yes |
| `or_default_interface` | Provide defaults for missing interface fields | `bool` | `false` | Yes |
| `or_default_list` | Provide defaults for missing list/array fields | `bool` | `false` | Yes |
| `or_default_set` | Provide defaults for missing set fields | `bool` | `false` | Yes |
| `or_default_map` | Provide defaults for missing map fields | `bool` | `false` | Yes |
| `or_default_generic` | Provide defaults for missing generic type fields | `bool` | `false` | Yes |

### Container Usage Example

```rust,ignore
#[grost(
  default = "MyObject::new",
  schema(name = "my_object", description = "A sample object"),
  flavor(type = "CustomFlavor", wire_format = "CustomWireFormat"),
  partial(rename = "MyPartialObject", copy = true),
  ref(rename = "MyObjectRef"),
  selector(rename = "MyObjectSelector"),
  or_default,
  or_default_scalar = false,
)]
struct MyObject {
  // fields...
}
```

## Container Schema Attribute

The schema attribute provides metadata for documentation and identification within the schema file.

| Attribute | Description | Type | Default | Optional |
|:---------:|-------------|------|---------|:--------:|
| `name` | A unique identifier for the object within the schema namespace. Used for cross-references and code generation | `String` | Same as Rust struct name | Yes |
| `description` | Human-readable documentation explaining the object's purpose, contents, or usage patterns | `String` | Extracted from Rust doc comments | Yes |

### Container Schema Examples

```rust,ignore
/// A user profile containing account information
#[grost(schema(name = "UserInformation"))]
struct UserProfile {
  // The description will be extracted from doc comments
}

#[grost(schema(
  name = "Awesome",
  description = "My awesome doc"
))]
struct AwesomeObject {
  // fields...
}
```

## Flavor Attribute

Registers a custom flavor that defines the information of the custom flavor, the generated code will use the registed flavor as the flavor type instead of the default one.

| Attribute | Description | Type | Default | Optional |
|:---------:|-------------|------|---------|:--------:|
| `type` | The custom flavor type to use for this object | `String` | N/A | No |
| `tag` | Configuration for tag creation and encoding functions | [`Tag`](#tag-attribute) | N/A | No |
| `identifier` | Configuration for identifier creation and encoding functions | [`Identifier`](#identifier-attribute) | N/A | No |
| `wire_format` | The path to the wire format type used for serialization | `String` | N/A | No |

### Tag Attribute

Defines const functions for creating and encoding tags in the custom flavor.

| Attribute | Description | Type | Default | Optional |
|:---------:|-------------|------|---------|:--------:|
| `constructor` | Const function or const closure to create a tag from `u32` | `String \| Closure` | N/A | No |
| `encode` | Const function or const closure to encode a tag to `&'static [u8]` | `String \| Closure` | N/A | No |

### Identifier Attribute

Defines const functions for creating and encoding identifiers in the custom flavor.

| Attribute | Description | Type | Default | Optional |
|:---------:|-------------|------|---------|:--------:|
| `constructor` | Const function or const closure to create an identifier from wire type and tag | `String \| Closure` | N/A | No |
| `encode` | Const function or const closure to encode an identifier to any type has a const method `as_slice` which returns a `&'static [u8]` | `String \| Closure` | N/A | No |

### Flavor Examples

```rust,ignore
#[grost(flavor(
  type = "MyCustomFlavor",
  tag(
    constructor = "create_my_tag",
    encode = "MyCustomFlavor::encode_tag"
  ),
  identifier(
    constructor = "MyCustomFlavor::create_identifier", 
    encode = "MyCustomFlavor::encode_identifier"
  ),
  wire_format = "MyCustomWireFormat"
))]
struct CustomObject {
  // fields...
}
```

## Generic Attribute

Customizes the names of generic parameters used in generated code to avoid conflicts with user-defined generics.

| Attribute | Description | Type | Default | Optional |
|:---------:|-------------|------|---------|:--------:|
| `lifetime` | Name of the lifetime generic parameter used in generated code | `String` | `__grost_lifetime__` | Yes |
| `read` | Name of the read buffer type generic parameter for deserialization | `String` | `__GROST_READ_BUF__` | Yes |
| `write` | Name of the write buffer type generic parameter for serialization | `String` | `__GROST_WRITE_BUF__` | Yes |
| `unknown` | Name of the unknown buffer type generic parameter for handling unknown fields | `String` | `__GROST_UNKNOWN_BUFFER__` | Yes |

### Generic Examples

```rust,ignore
// Use custom generic parameter names
#[grost(generic(
    lifetime = "custom_lifetime",
    read = "Buf",
    write = "ChunkMut"
))]
struct MyObject {
  // fields...
}
```

## Object Variant Attributes

Grost will generate several variants of your object for different use cases. Each variant shares common configuration patterns.

### Common Variant Configuration

All object variants support these common attributes:

| Attribute | Description | Type | Default | Optional |
|:---------:|-------------|------|---------|:--------:|
| `rename` | Custom name for the generated variant | `String` | See individual variants | Yes |
| `attrs` | Additional attributes to apply to the generated variant | `Vec<Attribute>` | `[]` | Yes |
| `copy` | Hint indicating all fields are copyable (can be overridden per field) | `bool` | `false` | Yes |

### Partial Attribute

Configures generation of partial objects.

| Attribute | Description | Type | Default | Optional |
|:---------:|-------------|------|---------|:--------:|
| `rename` | Custom name for the partial object | `String` | `Partial{ObjectName}` | Yes |
| `attrs` | Additional attributes for the partial object | `Vec<Attribute>` | `[]` | Yes |
| `copy` | Hint for copyable fields | `bool` | `false` | Yes |

**Plus all [type-specific default value attributes](#global-default-value-handling)**

### PartialRef Attribute

Configures generation of partial reference objects.

| Attribute | Description | Type | Default | Optional |
|:---------:|-------------|------|---------|:--------:|
| `rename` | Custom name for the partial reference object | `String` | `Partial{ObjectName}Ref` | Yes |
| `attrs` | Additional attributes for the partial reference object | `Vec<Attribute>` | `[]` | Yes |
| `copy` | Hint for copyable fields | `bool` | `false` | Yes |

**Plus all [type-specific default value attributes](#global-default-value-handling)**

### Ref Attribute

Configures generation of reference objects.

| Attribute | Description | Type | Default | Optional |
|:---------:|-------------|------|---------|:--------:|
| `rename` | Custom name for the reference object | `String` | `{ObjectName}Ref` | Yes |
| `attrs` | Additional attributes for the reference object | `Vec<Attribute>` | `[]` | Yes |
| `copy` | Hint for copyable fields | `bool` | `false` | Yes |

**Plus all [type-specific default value attributes](#global-default-value-handling)**

### Selector Attribute

Configures generation of the selectors of all object variants.

| Attribute | Description | Type | Default | Optional |
|:---------:|-------------|------|---------|:--------:|
| `rename` | Custom name for the generated selector | `String` | `{ObjectName}Selector` | Yes |
| `attrs` | Additional attributes for the selector | `Vec<Attribute>` | `[]` | Yes |

### Indexer Attribute

Configures generation of indexers for efficient data lookup operations.

| Attribute | Description | Type | Default | Optional |
|:---------:|-------------|------|---------|:--------:|
| `rename` | Custom name for the generated indexer | `String` | `{ObjectName}Indexer` | Yes |
| `attrs` | Additional attributes for the indexer | `Vec<Attribute>` | `[]` | Yes |

---

# Field attributes

Field attributes configure individual fields within Groto objects, providing fine-grained control over serialization, type handling, and code generation.

## Core Field Configuration

| Attribute | Description | Type | Default | Optional |
|:---------:|-------------|------|---------|:--------:|
| `tag` | Unique identifier for the field in the serialized format | `NonZeroU32` | N/A | No |
| label¹ | Type specification for the field (see [Label Types](#label-types)) | [`Label`](#label-types) | N/A | No |
| `schema` | Metadata for the field including name and documentation | [`Schema`](#field-schema-attribute) | See [`Schema`](#field-schema-attribute) | Yes |

¹ *The label is specified as one of the label variants (e.g., `scalar`, `string`, `object`, etc.), not as `label = ...`*

## Field Object Variant Configuration

| Attribute | Description | Type | Default | Optional |
|:---------:|-------------|------|---------|:--------:|
| `partial` | Configuration for this field in generated partial objects | [`PartialField`](#partial-field-attribute) | See [`PartialField`](#partial-field-attribute) | Yes |
| `partial_ref` | Configuration for this field in generated partial reference objects | [`PartialRefField`](#partial-ref-field-attribute) | See [`PartialRefField`](#partial-ref-field-attribute) | Yes |
| `ref` | Configuration for this field in generated reference objects | [`RefField`](#ref-field-attribute) | See [`RefField`](#ref-field-attribute) | Yes |
| `selector` | Configuration for this field in generated selectors | [`SelectorField`](#selector-field-attribute) | See [`SelectorField`](#selector-field-attribute) | Yes |

## Field Serialization Configuration

| Attribute | Description | Type | Default | Optional |
|:---------:|-------------|------|---------|:--------:|
| `encode` | Custom encoding configuration for this field | [`Encode`](#encode-field-attribute) | See [`Encode`](#encode-field-attribute) | Yes |
| `decode` | Custom decoding configuration for this field | [`Decode`](#decode-field-attribute) | See [`Decode`](#decode-field-attribute) | Yes |

## Field Default Value Handling

Field-level default value handling provides the highest precedence in the [default value hierarchy](#default-value-precedence).

| Attribute | Description | Type | Default | Optional |
|:---------:|-------------|------|---------|:--------:|
| `or_default` | Use default value when field is missing (mutually exclusive with `or_else` and `ok_or_else`) | `bool \| N/A` | N/A | Yes |
| `or_else` | Function/closure to call when field is missing (mutually exclusive with others) | `String \| Closure` | N/A | Yes |
| `ok_or_else` | Function/closure for error handling when field is missing (mutually exclusive with others) | `String \| Closure` | N/A | Yes |
| `copy` | Hint that this field is copyable (overrides container-level copy settings) | `bool \| N/A` | `false` | Yes |

### Field Usage Examples

```rust,ignore
#[grost(/* container attributes */)]
struct User {
  #[grost(tag = 1, scalar, schema(name = "user_id"))]
  id: u64,
  
  #[grost(tag = 2, string, or_default)]
  name: String,
  
  #[grost(tag = 3, nullable(scalar), or_else = "default_age")]
  age: Option<u32>,
  
  #[grost(tag = 4, list(string), copy)]
  tags: Vec<String>,

  #[grost(tag = 4, list(string, repeated), copy)]
  friends: Vec<String>,

  #[grost(tag = 5, object(as = "CustomUserProfile"))]
  profile: UserProfile,
}
```

## Field Schema Attribute

Provides metadata for individual fields.

| Attribute | Description | Type | Default | Optional |
|:---------:|-------------|------|---------|:--------:|
| `name` | Unique identifier for the field within the object schema | `String` | Same as field name | Yes |
| `description` | Human-readable documentation for the field's purpose and usage | `String` | Extracted from field doc comments | Yes |

### Field Schema Examples

```rust,ignore
struct User {
  /// User's unique identifier
  #[grost(tag = 1, scalar, schema(name = "user_id"))]
  id: u64,
  
  #[grost(tag = 2, string, schema(
      name = "display_name",
      description = "User's preferred display name"
  ))]
  name: String,
}
```

## Label Types

The label system provides comprehensive type specification for fields. Each label type corresponds to a category of data with specific serialization behavior.

### Simple Labels

| Label | Description | Configuration |
|:-----:|-------------|---------------|
| `scalar` | Primitive types (numbers, booleans) | [`ScalarLabel`](#scalar-label) |
| `string` | Text data | [`StringLabel`](#string-label) |
| `bytes` | Binary data | [`BytesLabel`](#bytes-label) |
| `object` | Nested objects | [`ObjectLabel`](#object-label) |
| `enum` | Enumeration types | [`EnumLabel`](#enum-label) |
| `union` | Union types | [`UnionLabel`](#union-label) |
| `interface` | Interface types | [`InterfaceLabel`](#interface-label) |

### Container Labels

| Label | Description | Configuration |
|:-----:|-------------|---------------|
| `list` | List-like collections | [`ListLabel`](#list-label) |
| `set` | Unique collections | [`SetLabel`](#set-label) |
| `map` | Key-value associations | [`MapLabel`](#map-label) |
| `nullable` | Optional values | [`NullableLabel`](#nullable-label) |

### Generic Label

| Label | Description | Configuration |
|:-----:|-------------|---------------|
| `generic` | Generic types | [`GenericLabel`](#generic-label) |

### Label Configuration

Most labels support wire format customization:

| Attribute | Description | Type | Default | Optional |
|:---------:|-------------|------|---------|:--------:|
| `as` | Custom wire format type for encoding/decoding | `String` | Type-specific default | Yes |

### Simple Label Types

#### Scalar Label

For primitive scalar types (numbers, booleans, etc.).

```rust,ignore
struct Example {
  #[grost(tag = 1, scalar)]
  count: u32,
  
  #[grost(tag = 2, scalar(as = "Fixed64"))]
  custom_id: u64,
}
```

#### String Label

For text data types.

```rust,ignore
struct Example {
  #[grost(tag = 1, string)]
  name: String,
  
  #[grost(tag = 2, string(as = "CustomStringFormat"))]
  description: String,
}
```

#### Bytes Label

For binary data types.

```rust,ignore
struct Example {
  #[grost(tag = 1, bytes)]
  data: Vec<u8>,
  
  #[grost(tag = 2, bytes(as = "Fixed128"))]
  signature: [u8; 32],
}
```

#### Object Label

For nested object types.

```rust,ignore
struct Example {
  #[grost(tag = 1, object)]
  profile: UserProfile,
  
  #[grost(tag = 2, object(as = "CustomProfileFormat"))]
  settings: UserSettings,
}
```

#### Enum Label

For enumeration types.

```rust,ignore
struct Example {
  #[grost(tag = 1, enum)]
  status: UserStatus,
  
  #[grost(tag = 2, enum(as = "CustomEnumFormat"))]
  role: UserRole,
}
```

#### Union Label

For union types.

```rust,ignore
struct Example {
  #[grost(tag = 1, union)]
  value: MyUnion,
  
  #[grost(tag = 2, union(as = "CustomUnionFormat"))]
  data: DataUnion,
}
```

#### Interface Label

For interface types.

```rust,ignore
struct Example {
  #[grost(tag = 1, interface)]
  handler: Box<dyn EventHandlerInterface>,
  
  #[grost(tag = 2, interface(as = "CustomInterfaceFormat"))]
  processor: Box<dyn DataProcessorInterface>,
}

struct StaticDispatchExample {
  #[grost(tag = 1, interface)]
  handler: EventHandlerInterfaceEnum,

  #[grost(tag = 2, interface(as = "CustomInterfaceFormat"))]
  processor: DataProcessorInterfaceEnum,
}
```

### Container Label Types

Container labels require inner type specifications.

#### List Label

For ordered collections. Requires inner type specification.

| Attribute | Description | Type | Default | Optional |
|:---------:|-------------|------|---------|:--------:|
| label¹ | Specification of the element type | [`Label`](#label-types) | N/A | No² |
| `repeated` | Mark the list as using repeated encoding (can be used with `label`) | N/A | N/A | Yes |
| `as` | Custom wire format type | `String` | Default list format | Yes |

¹ *The label is specified as one of the label variants (e.g., `scalar`, `string`, `object`, etc.), not as `label = ...`*

² *`label` and `repeated` can be used together, but both are mutually exclusive with `as`*

```rust,ignore
struct Example {
  #[grost(tag = 1, list(string))]
  names: Vec<String>,
  
  #[grost(tag = 2, list(scalar))]
  numbers: Vec<u32>,
  
  #[grost(tag = 3, list(object))]
  users: Vec<User>,
  
  #[grost(tag = 4, list(string, repeated))]
  repeated_names: Vec<String>,
  
  #[grost(tag = 5, list(as = "CustomListFormat"))]
  custom_data: Vec<CustomType>,
}
```

#### Set Label

For unordered unique collections.

| Attribute | Description | Type | Default | Optional |
|:---------:|-------------|------|---------|:--------:|
| label¹ | Specification of the element type | [`Label`](#label-types) | N/A | No² |
| `repeated` | Mark the set as using repeated encoding (can be used with `label`) | N/A | N/A | Yes |
| `as` | Custom wire format type | `String` | Default set format | Yes |

¹ *The label is specified as one of the label variants (e.g., `scalar`, `string`, `object`, etc.), not as `label = ...`*

² *`label` and `repeated` can be used together, but both are mutually exclusive with `as`*

```rust,ignore
struct Example {
  #[grost(tag = 1, set(string))]
  tags: HashSet<String>,
  
  #[grost(tag = 2, set(scalar))]
  ids: HashSet<u64>,
  
  #[grost(tag = 3, set(bytes, repeated))]
  repeated_data: HashSet<Vec<u8>>,
  
  #[grost(tag = 4, set(as = "CustomSetFormat"))]
  custom_set: HashSet<CustomType>,
}
```

#### Map Label

For key-value associations. Requires both key and value type specifications.

| Attribute | Description | Type | Default | Optional |
|:---------:|-------------|------|---------|:--------:|
| `key` | Specification of the key type | [`Label`](#label-types) | N/A | No² |
| `value` | Specification of the value type | [`Label`](#label-types) | N/A | No² |
| `repeated` | Mark the map as using repeated encoding (can be used with `key` and `value`) | N/A | N/A | Yes |
| `as` | Custom wire format type | `String` | Default map format | Yes |

² *`key`, `value`, and `repeated` can be used together, but all are mutually exclusive with `as`*

```rust,ignore
struct Example {
  #[grost(tag = 1, map(key(string), value(scalar)))]
  scores: HashMap<String, u32>,
  
  #[grost(tag = 2, map(key(scalar), value(object)))]
  user_profiles: HashMap<u64, UserProfile>,
  
  #[grost(tag = 3, map(key(string), value(string), repeated))]
  repeated_mapping: HashMap<String, String>,
  
  #[grost(tag = 4, map(as = "CustomMapFormat"))]
  custom_mapping: HashMap<CustomKey, CustomValue>,
}
```

#### Nullable Label

For optional values. Configuration identical to [`ListLabel`](#list-label).

| Attribute | Description | Type | Default | Optional |
|:---------:|-------------|------|---------|:--------:|
| label¹ | Specification of the inner type | [`Label`](#label-types) | N/A | No² |
| `as` | Custom wire format type | `String` | Default option format | Yes |

¹ *The label is specified as one of the label variants (e.g., `scalar`, `string`, `object`, etc.), not as `label = ...`*

² *`label` and `repeated` can be used together, but both are mutually exclusive with `as`*

```rust,ignore
struct Example {
  #[grost(tag = 1, nullable(string))]
  optional_name: Option<String>,
  
  #[grost(tag = 2, nullable(object))]
  optional_profile: Option<UserProfile>,
  
  #[grost(tag = 3, nullable(as = "CustomOptionFormat"))]
  custom_optional: Option<CustomType>,
}
```

### Generic Label Types

For generic types with type markers that hint at the actual type during serialization.

| Attribute | Description | Type | Default | Optional |
|:---------:|-------------|------|---------|:--------:|
| `marker` | The marker type which hints at the actual type label | `String` | N/A | No¹ |
| `marked` | The type that is marked (required when generic is wrapped by a container label) | `String` | N/A | Yes² |
| `as` | Custom wire format type | `String` | Default generic format | Yes³ |

¹ *The `marker` attribute is always required for generic labels*

² *Required when the generic is wrapped by a container label (e.g., in `list(generic(...))` or `nullable(generic(...))`)*

³ *`marker` and `marked` cannot be used together with `as` - they are mutually exclusive*

#### Generic Label Example

```rust,ignore
/// Generic over dynamic id types
#[derive(Object)]
struct GenericWithMarker<I, M> {
  #[grost(tag = 1, generic(marker = "M"))]
  id: I,
  #[grost(skip)]
  _m: PhantomData<M>,
}

/// Generic over bytes id types
#[derive(Object)]
struct GenericWithKnownMarker<I> {
  #[grost(tag = 1, generic(marker = "BytesMarker<I>"))]
  id: I,
}

/// Generic over nullable dynamic id types
#[derive(Object)]
struct GenericWithNullableMarker<I, M> {
  #[grost(tag = 1, nullable(generic(
    marker = "NullableMarker<Option<I>, M>", 
    marked = "I"
  )))]
  id: Option<I>,
  #[grost(skip)]
  _m: PhantomData<M>,
}

/// Generic list with marked type
#[derive(Object)]
struct GenericList<T, M> {
  #[grost(tag = 1, list(generic(
    marker = "ListMarker<T, M>",
    marked = "T"
  )))]
  items: Vec<T>,
  #[grost(skip)]
  _m: PhantomData<M>,
}
```

## Nested Label Examples

Labels can be nested to represent complex types, and container labels can use the `repeated` attribute:

```rust,ignore
struct ComplexExample {
  // List of optional strings
  #[grost(tag = 1, list(nullable(string)))]
  optional_names: Vec<Option<String>>,
  
  // Repeated list of strings
  #[grost(tag = 2, list(string, repeated))]
  repeated_tags: Vec<String>,
  
  // Map from strings to lists of user objects
  #[grost(tag = 3, map(
      key(string),
      value(list(object))
  ))]
  user_groups: HashMap<String, Vec<User>>,
  
  // Repeated map with scalar keys and string values
  #[grost(tag = 4, map(key(scalar), value(string), repeated))]
  repeated_config: HashMap<u32, String>,
  
  // Optional map of string to optional numbers
  #[grost(tag = 5, nullable(map(
    key(string),
    value(nullable(scalar))
  )))]
  optional_scores: Option<HashMap<String, Option<u32>>>,
  
  // Set with repeated encoding
  #[grost(tag = 6, set(bytes, repeated))]
  repeated_hashes: HashSet<Vec<u8>>,
}
```

## Field Variant Attributes

These attributes configure how fields behave in different generated object variants.

### Partial Field Attribute

Configures field behavior in generated partial objects.

| Attribute | Description | Type | Default | Optional |
|:---------:|-------------|------|---------|:--------:|
| `type` | Custom type for the field in the partial object | `String` | N/A | Yes |
| `attrs` | Additional attributes for the field in the partial object | `Vec<Attribute>` | `[]` | Yes |
| `from_ref` | Configuration for converting from reference object field | [`Convert`](#convert-field-attribute) | See [`Convert`](#convert-field-attribute) | Yes |
| `from_partial_ref` | Configuration for converting from partial reference object field | [`Convert`](#convert-field-attribute) | See [`Convert`](#convert-field-attribute) | Yes |
| `encode` | Custom encoding configuration for the partial object field | [`Encode`](#encode-field-attribute) | See [`Encode`](#encode-field-attribute) | Yes |
| `decode` | Custom decoding configuration for the partial object field | [`Decode`](#decode-field-attribute) | See [`Decode`](#decode-field-attribute) | Yes |

**Plus all [field-level default value attributes](#field-default-value-handling)**

### Partial Ref Field Attribute

Configures field behavior in generated partial reference objects.

| Attribute | Description | Type | Default | Optional |
|:---------:|-------------|------|---------|:--------:|
| `type` | Custom type for the field in the partial reference object | `String` | N/A | Yes |
| `attrs` | Additional attributes for the field in the partial reference object | `Vec<Attribute>` | `[]` | Yes |
| `encode` | Custom encoding configuration for the partial reference field | [`Encode`](#encode-field-attribute) | See [`Encode`](#encode-field-attribute) | Yes |
| `decode` | Custom decoding configuration for the partial reference field | [`Decode`](#decode-field-attribute) | See [`Decode`](#decode-field-attribute) | Yes |

**Plus all [field-level default value attributes](#field-default-value-handling)**

### Ref Field Attribute

Configures field behavior in generated reference objects.

| Attribute | Description | Type | Default | Optional |
|:---------:|-------------|------|---------|:--------:|
| `type` | Custom type for the field in the reference object | `String` | N/A | Yes |
| `attrs` | Additional attributes for the field in the reference object | `Vec<Attribute>` | `[]` | Yes |
| `encode` | Custom encoding configuration for the reference field | [`Encode`](#encode-field-attribute) | See [`Encode`](#encode-field-attribute) | Yes |
| `decode` | Custom decoding configuration for the reference field | [`Decode`](#decode-field-attribute) | See [`Decode`](#decode-field-attribute) | Yes |

**Plus all [field-level default value attributes](#field-default-value-handling)**

### Selector Field Attribute

Configures field behavior in generated selectors.

| Attribute | Description | Type | Default | Optional |
|:---------:|-------------|------|---------|:--------:|
| `attrs` | Additional attributes for the field in the selector | `Vec<Attribute>` | `[]` | Yes |
| `select` | Default selection behavior for this field | [`FieldSelection`](#field-selection-attribute) | Default selection for each field | Yes |

### Field Selection Attribute

Defines default selection behavior for fields in selectors.

| Variant | Description | Type |
|:---------:|-------------|------|
| `default` | Use the default selection for each field | N/A |
| `all` | Select all fields by default | N/A |
| `none` | Select no fields by default | N/A |
| `custom` | Use a custom function/closure to determine selection | `String \| Closure` |

## Field Serialization Attributes

### Encode Field Attribute

Configures custom encoding behavior for fields.

| Attribute | Description | Type | Default | Optional |
|:---------:|-------------|------|---------|:--------:|
| `skip` | Skip encoding this field entirely (mutually exclusive with `skip_if`) | `bool` | `false` | Yes |
| `skip_if` | Skip encoding if the function/closure returns `true` (mutually exclusive with `skip`) | `String \| Closure` | N/A | Yes |
| `error_if` | Return an error if the function/closure returns `Err(_)` | `String \| Closure` | N/A | Yes |

### Decode Field Attribute

Configures custom decoding behavior for fields.

| Attribute | Description | Type | Default | Optional |
|:---------:|-------------|------|---------|:--------:|
| `fn` | Custom function for decoding this field | `String \| Closure` | Default decode implementation | Yes |
| `then` | Function/closure to call after successful decoding | `String \| Closure` | N/A | Yes |

### Convert Field Attribute

Configures field conversion between object variants.

| Attribute | Description | Type | Default | Optional |
|:---------:|-------------|------|---------|:--------:|
| `from` | Function for converting between variants (mutually exclusive with `try_from`) | `String \| Closure` | N/A | Yes |
| `try_from` | Fallible function for converting between variants (mutually exclusive with `from`) | `String \| Closure` | N/A | Yes |

### Serialization Examples

```rust,ignore
struct User {
  #[grost(tag = 1, scalar)]
  id: u64,
  
  #[grost(
    tag = 2, 
    string,
    encode(skip_if = "String::is_empty"),
    decode(then = "validate_name")
  )]
  name: String,
  
  #[grost(
    tag = 3,
    scalar,
    encode(error_if = "check_age_valid"),
    partial(from_ref(try_from = "age_from_string"))
  )]
  age: u32,
  
  #[grost(
    tag = 4,
    list(label(string)),
    encode(skip)  // Never encode this field
  )]
  internal_tags: Vec<String>,
}
```

## Default Value Precedence

Default value handling follows this precedence order (highest to lowest):

1. **Field-level attributes** - `or_default`, `or_else`, `ok_or_else` on individual fields
2. **Field variant attributes** - Default value attributes in `partial`, `partial_ref`, `ref` field configurations
3. **Object variant attributes** - `or_default_*` attributes on container `partial`, `partial_ref`, or `ref`
4. **Type-specific container attributes** - `or_default_scalar`, `or_default_string`, etc.
5. **Global container attribute** - `or_default` on the container
6. **System default** - `false` (no default values provided)

## Complete Example

```rust,ignore
/// A comprehensive user profile object
/// 
/// This object contains all user account information
/// including personal details and preferences.
#[grost(
  default = "UserProfile::empty",
  schema(name = "user_profile"),
  generic(lifetime = "profile_lifetime", read = "RB"),
  partial(
    rename = "UserProfilePartial",
    copy,
    or_default_string,
    or_default_scalar,
    attrs(derive(Debug, Clone))
  ),
  partial_ref(
    rename = "UserProfilePartialRef",
    or_default_string,
  ),
  ref(
    rename = "UserProfileRef",
    attrs(derive(Debug))
  ),
  selector(
    rename = "UserProfileSelector",
    attrs(derive(Debug, Clone))
  ),
  indexer(rename = "UserProfileIndexer"),
  or_default_list,
)]
struct UserProfile {
  /// Unique user identifier
  #[grost(
    tag = 1, 
    scalar, 
    schema(name = "user_id", description = "Primary key for user identification")
  )]
  id: u64,
  
  /// User's display name
  #[grost(
    tag = 2, 
    string, 
    or_default = true,
    encode(skip_if = "String::is_empty"),
    partial(
      type = "Option<String>",
    ),
    selector(selection = all)
  )]
  name: String,
  
  /// Optional user age
  #[grost(
    tag = 3, 
    nullable(scalar), 
    or_else = "default_age",
    decode(then = "validate_age"),
    partial(
      type = "Option<u32>",
      attrs(serde(skip_serializing_if = "Option::is_none"))
    )
  )]
  age: Option<u32>,
  
  /// User tags for categorization
  #[grost(
    tag = 4, 
    list(string), 
    copy = true,
    encode(skip_if = "Vec::is_empty"),
    partial(from_ref(from = "Vec::clone")),
    selector(selection = default)
  )]
  tags: Vec<String>,
  
  /// User preferences mapping
  #[grost(
    tag = 5, 
    map(key = string, value = string),
    or_default = true,
    partial(
      type = "HashMap<String, String>",
    ),
    ref(type = "HashMap<Str<RB>, Str<RB>>")
  )]
  preferences: HashMap<String, String>,
  
  /// Nested profile data
  #[grost(
    tag = 6, 
    object,
    decode(fn = "custom_profile_decode"),
    partial(
      type = "Option<ProfileDataPartial>",
    ),
    ref(type = "ProfileDataRef<'profile_lifetime>"),
    selector(select(all))
  )]
  profile_data: ProfileData,
  
  /// Optional metadata with custom encoding
  #[grost(
    tag = 7,
    nullable(map(key(string), value(bytes))),
    encode(skip_if = "Option::is_none"),
    decode(then = "process_metadata"),
    partial(
      or_default = true,
      from_partial_ref(from = "clone_metadata")
    )
  )]
  metadata: Option<HashMap<String, Vec<u8>>>,
  
  /// Internal field that's never encoded
  #[grost(
    tag = 8,
    list(string),
    encode(skip),
    partial(type = "Vec<String>"),
  )]
  internal_notes: Vec<String>,
}

/// Supporting profile data structure
#[grost(
  schema(name = "profile_data"),
  partial(rename = "ProfileDataPartial")
)]
struct ProfileData {
  #[grost(tag = 1, string)]
  bio: String,
  
  #[grost(tag = 2, nullable(string))]
  website: Option<String>,

  #[grost(tag = 3, scalar)]
  created_at: u64,
}
```

This comprehensive example demonstrates:

### Generated Objects

- `UserProfile` - The main object
- `UserProfilePartial` - For optional field handling during deserialization
- `UserProfilePartialRef` - For borrowed optional field handling  
- `UserProfileRef` - For borrowed data access
- `UserProfileSelector` - For field selection patterns
- `UserProfileIndexer` - For efficient data lookup

### Advanced Features Showcased

- **Complex type labels** with nested specifications
- **Cross-variant field conversion** with custom functions
- **Conditional encoding** based on field values
- **Custom decoding** with post-processing
- **Field-specific type overrides** in different variants
- **Comprehensive default value handling** at multiple levels
- **Selector configuration** for data access patterns
- **Schema documentation** integration

### Default Value Hierarchy in Action

1. Field-level `or_default` on `name` (highest precedence)
2. Field-level `or_else = "default_age"` on `age`
3. Partial field-level `or_default` on `metadata.partial`
4. Container-level `or_default_string` affects string fields
5. Container-level `or_default_list = false` affects list fields

### Encoding/Decoding Customization

- **Conditional encoding**: Skip empty strings and vectors
- **Custom decoding**: Validate age and process metadata
- **Field conversion**: Handle type transformations between variants
- **Skip encoding**: Internal fields never serialized

This example provides a complete reference for using Groto objects with maximum flexibility and type safety.

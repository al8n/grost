Define a Groto object

# Container attributes

| Attribute | Description | Type | Default | Optional |
|:---------:|-------------|------|---------|:--------:|
| default   | Specify the default fn to create an object (If not provided, the code will not generate `Default` implementation for the object) | `String \| Closure` | `None` | Y |
| schema    | Defines metadata for the object including its name and descriptive information | [`Schema`](#schema-attribute) | See [`Schema`](#schema-attribute) | Y |
| generic   | Rename the generic parameters will be used in the generated code| [`Generic`](#generic-attribute) | See [`Generic`](#generic-attribute) | Y |
| flavor   | Register a custom flavor, the generated code will concrete to the registered flavor instead of the default flavor | [`Flavor`](#flavor-attribute) | See [`Flavor`](#flavor-attribute) | Y |
| partial | The options for configuring the generated partial object | [`Partial`](#partial-attribute) | See [`Partial`](#partial-attribute) | Y |
| partial_ref | The options for configuring the generated partial reference object | [`PartialRef`](#partial-ref-attribute) | See [`PartialRef`](#partial-ref-attribute) | Y |
| ref | The options for configuring the generated reference object | [`Ref`](#ref-attribute) | See [`Ref`](#ref-attribute) | Y |
| selector | The options for configuring the generated selector of the object | [`Selector`](#selector-attribute) | See [`Selector`](#selector-attribute) | Y |
| indexer | The options for configuring the generated indexer of the object | [`Indexer`](#indexer-attribute) | See [`Indexer`](#indexer-attribute) | Y |
| or_default | Specify when a field is missing while decoding or converting, a default value should be given (can be overwritten by field level attributes and label `or_default_*` attribute specification) | `bool` | `false` | Y |
| or_default_scalar | Specify when fields with `scalar` label are missing while decoding or converting, a default value should be given (can be overwritten by field level specification) | `bool` | `false` | Y |
| or_default_string | Specify when fields with `string` label are missing while decoding or converting, a default value should be given (can be overwritten by field level specification) | `bool` | `false` | Y |
| or_default_bytes | Specify when fields with `bytes` label are missing while decoding or converting, a default value should be given (can be overwritten by field level specification) | `bool` | `false` | Y |
| or_default_object | Specify when fields with `object` label are missing while decoding or converting, a default value should be given (can be overwritten by field level specification) | `bool` | `false` | Y |
| or_default_enum | Specify when fields with `enum` label are missing while decoding or converting, a default value should be given (can be overwritten by field level specification) | `bool` | `false` | Y |
| or_default_union | Specify when fields with `union` label are missing while decoding or converting, a default value should be given (can be overwritten by field level specification) | `bool` | `false` | Y |
| or_default_interface | Specify when fields with `interface` label are missing while decoding or converting, a default value should be given (can be overwritten by field level specification) | `bool` | `false` | Y |
| or_default_list | Specify when fields with `list` label are missing while decoding or converting, a default value should be given (can be overwritten by field level specification) | `bool` | `false` | Y |
| or_default_set | Specify when fields with `set` label are missing while decoding or converting, a default value should be given (can be overwritten by field level specification) | `bool` | `false` | Y |
| or_default_map | Specify when fields with `map` label are missing while decoding or converting, a default value should be given (can be overwritten by field level specification) | `bool` | `false` | Y |
| or_default_generic | Specify when fields with `generic` label are missing while decoding or converting, a default value should be given (can be overwritten by field level specification) | `bool` | `false` | Y |

## Schema Attribute

Defines metadata for the object including its name and descriptive information

| Attribute | Description                           | Type   | Default | Optional |
|:---------:|---------------------------------------|--------|---------|:--------:|
| name      | A unique identifier in the schema file | `String` | The same as the struct name in Rust | Y        |
| description | Human-readable documentation that explains the purpose, contents, or usage | `String` | The same as the doc comments in Rust | Y |

## Flavor Attribute

Register a custom flavor so that the generated code is concrete to the custom flavor instead of the default flavor

| Attribute | Description                           | Type   | Default | Optional |
|:---------:|---------------------------------------|--------|---------|:--------:|
| type      | The flavor type | `String` | N/A | N        |
| tag | The path to const functions used to create or encode a tag of the registered flavor | [`Tag`](#tag-attribute) | N/A | N |
| identifier | The path to const functions used to create or encode an identifier of the registered flavor | [`Identifier`](#identifier-attribute) | N/A | N |
| wire_format | The path to the wire format type for the object | `String` | N/A | N |

### Tag Attribute

Pathes to const functions used to create or encode a tag of the registered flavor

| Attribute | Description                           | Type   | Default | Optional |
|:---------:|---------------------------------------|--------|---------|:--------:|
| constructor      | The const fn or a const closure to create a tag from `u32` | `String \| Closure` | N/A | N |
| encode | The const fn or a const closure to encode a tag to `&'static [u8]` | `String \| Closure` | N/A | N |

### Identifier Attribute

Pathes to const functions used to create or encode a tag of the registered flavor

| Attribute | Description                           | Type   | Default | Optional |
|:---------:|---------------------------------------|--------|---------|:--------:|
| constructor      | The const fn or a const closure to create an identifier by wire type and tag | `String \| Closure` | N/A | N |
| encode | The const fn or a const closure to encode an identifier to `&'static [u8]` | `String \| Closure` | N/A | N |

## Generic Attribute

Defines the name of the generic parameters used in the generated code

| Attribute | Description                           | Type   | Default | Optional |
|:---------:|---------------------------------------|--------|---------|:--------:|
| lifetime      | The name of the lifetime generic parameter | `String`  | `__grost_lifetime__` | Y        |
| read | The name of the read buffer type generic parameter | `String` | `__GROST_READ_BUF__` | Y |
| write | The name of the write buffer type generic parameter | `String` | `__GROST_WRITE_BUF__` | Y |
| unknown | The name of the unknown buffer type generic parameter | `String` | `__GROST_UNKNOWN_BUFFER__` | Y |

## Partial Attribute

The options for configuring the generated partial object

| Attribute | Description                           | Type     | Default     | Optional |
|:---------:|---------------------------------------|----------|-------------|:--------:|
| rename | Rename the generated partial object name | `String` | `format!("Partial{}", object_name)` | Y |
| attrs | Forwarded attributes for partial object | `Vec<Attribute>` | `[]` | Y |
| copy  | A hint for whether all fields is copyable, can be overwritten by field level `copy` attribute | `bool` | `false` | Y |
| or_default | Specify when a field is missing while decoding or converting, a default value should be given (can be overwritten by field level attributes and label `or_default_*` attribute specification) | `bool` | `false` | Y |
| or_default_scalar | Specify when fields with `scalar` label are missing while decoding or converting, a default value should be given (can be overwritten by field level specification) | `bool` | `false` | Y |
| or_default_string | Specify when fields with `string` label are missing while decoding or converting, a default value should be given (can be overwritten by field level specification) | `bool` | `false` | Y |
| or_default_bytes | Specify when fields with `bytes` label are missing while decoding or converting, a default value should be given (can be overwritten by field level specification) | `bool` | `false` | Y |
| or_default_object | Specify when fields with `object` label are missing while decoding or converting, a default value should be given (can be overwritten by field level specification) | `bool` | `false` | Y |
| or_default_enum | Specify when fields with `enum` label are missing while decoding or converting, a default value should be given (can be overwritten by field level specification) | `bool` | `false` | Y |
| or_default_union | Specify when fields with `union` label are missing while decoding or converting, a default value should be given (can be overwritten by field level specification) | `bool` | `false` | Y |
| or_default_interface | Specify when fields with `interface` label are missing while decoding or converting, a default value should be given (can be overwritten by field level specification) | `bool` | `false` | Y |
| or_default_list | Specify when fields with `list` label are missing while decoding or converting, a default value should be given (can be overwritten by field level specification) | `bool` | `false` | Y |
| or_default_set | Specify when fields with `set` label are missing while decoding or converting, a default value should be given (can be overwritten by field level specification) | `bool` | `false` | Y |
| or_default_map | Specify when fields with `map` label are missing while decoding or converting, a default value should be given (can be overwritten by field level specification) | `bool` | `false` | Y |
| or_default_generic | Specify when fields with `generic` label are missing while decoding or converting, a default value should be given (can be overwritten by field level specification) | `bool` | `false` | Y |

## Partial Ref Attribute

The options for configuring the generated partial reference object

| Attribute | Description                           | Type     | Default     | Optional |
|:---------:|---------------------------------------|----------|-------------|:--------:|
| rename | Rename the generated partial reference object name | `String` | `format!("Partial{}Ref", object_name)` | Y |
| attrs | Forwarded attributes for partial reference object | `Vec<Attribute>` | `[]` | Y |
| copy  | A hint for whether all fields is copyable, can be overwritten by field level `copy` attribute | `bool` | `false` | Y |
| or_default | Specify when a field is missing while decoding or converting, a default value should be given (can be overwritten by field level attributes and label `or_default_*` attribute specification) | `bool` | `false` | Y |
| or_default_scalar | Specify when fields with `scalar` label are missing while decoding or converting, a default value should be given (can be overwritten by field level specification) | `bool` | `false` | Y |
| or_default_string | Specify when fields with `string` label are missing while decoding or converting, a default value should be given (can be overwritten by field level specification) | `bool` | `false` | Y |
| or_default_bytes | Specify when fields with `bytes` label are missing while decoding or converting, a default value should be given (can be overwritten by field level specification) | `bool` | `false` | Y |
| or_default_object | Specify when fields with `object` label are missing while decoding or converting, a default value should be given (can be overwritten by field level specification) | `bool` | `false` | Y |
| or_default_enum | Specify when fields with `enum` label are missing while decoding or converting, a default value should be given (can be overwritten by field level specification) | `bool` | `false` | Y |
| or_default_union | Specify when fields with `union` label are missing while decoding or converting, a default value should be given (can be overwritten by field level specification) | `bool` | `false` | Y |
| or_default_interface | Specify when fields with `interface` label are missing while decoding or converting, a default value should be given (can be overwritten by field level specification) | `bool` | `false` | Y |
| or_default_list | Specify when fields with `list` label are missing while decoding or converting, a default value should be given (can be overwritten by field level specification) | `bool` | `false` | Y |
| or_default_set | Specify when fields with `set` label are missing while decoding or converting, a default value should be given (can be overwritten by field level specification) | `bool` | `false` | Y |
| or_default_map | Specify when fields with `map` label are missing while decoding or converting, a default value should be given (can be overwritten by field level specification) | `bool` | `false` | Y |
| or_default_generic | Specify when fields with `generic` label are missing while decoding or converting, a default value should be given (can be overwritten by field level specification) | `bool` | `false` | Y |

## Ref Attribute

The options for configuring the generated reference object

| Attribute | Description                           | Type     | Default     | Optional |
|:---------:|---------------------------------------|----------|-------------|:--------:|
| rename | Rename the generated reference object name | `String` | `format!("{}Ref", object_name)` | Y |
| attrs | Forwarded attributes for reference object | `Vec<Attribute>` | `[]` | Y |
| copy  | A hint for whether all fields is copyable, can be overwritten by field level `copy` attribute | `bool` | `false` | Y |
| or_default | Specify when a field is missing while decoding or converting, a default value should be given (can be overwritten by field level attributes and label `or_default_*` attribute specification) | `bool` | `false` | Y |
| or_default_scalar | Specify when fields with `scalar` label are missing while decoding or converting, a default value should be given (can be overwritten by field level specification) | `bool` | `false` | Y |
| or_default_string | Specify when fields with `string` label are missing while decoding or converting, a default value should be given (can be overwritten by field level specification) | `bool` | `false` | Y |
| or_default_bytes | Specify when fields with `bytes` label are missing while decoding or converting, a default value should be given (can be overwritten by field level specification) | `bool` | `false` | Y |
| or_default_object | Specify when fields with `object` label are missing while decoding or converting, a default value should be given (can be overwritten by field level specification) | `bool` | `false` | Y |
| or_default_enum | Specify when fields with `enum` label are missing while decoding or converting, a default value should be given (can be overwritten by field level specification) | `bool` | `false` | Y |
| or_default_union | Specify when fields with `union` label are missing while decoding or converting, a default value should be given (can be overwritten by field level specification) | `bool` | `false` | Y |
| or_default_interface | Specify when fields with `interface` label are missing while decoding or converting, a default value should be given (can be overwritten by field level specification) | `bool` | `false` | Y |
| or_default_list | Specify when fields with `list` label are missing while decoding or converting, a default value should be given (can be overwritten by field level specification) | `bool` | `false` | Y |
| or_default_set | Specify when fields with `set` label are missing while decoding or converting, a default value should be given (can be overwritten by field level specification) | `bool` | `false` | Y |
| or_default_map | Specify when fields with `map` label are missing while decoding or converting, a default value should be given (can be overwritten by field level specification) | `bool` | `false` | Y |
| or_default_generic | Specify when fields with `generic` label are missing while decoding or converting, a default value should be given (can be overwritten by field level specification) | `bool` | `false` | Y |

## Selector Attribute

The options for configuring the generated selector of the object

| Attribute | Description                           | Type     | Default     | Optional |
|:---------:|---------------------------------------|----------|-------------|:--------:|
| rename | Rename the generated selector name | `String` | `format!("{}Selector", object_name)` | Y |
| attrs | Forwarded attributes for the generated selector | `Vec<Attribute>` | `[]` | Y |

## Indexer Attribute

The options for configuring the generated indexer of the object

| Attribute | Description                           | Type     | Default     | Optional |
|:---------:|---------------------------------------|----------|-------------|:--------:|
| rename | Rename the generated indexer | `String` | `format!("{}Indexer", object_name)` | Y |
| attrs | Forwarded attributes for the generated indexer | `Vec<Attribute>` | `[]` | Y |

# Field attributes

| Attribute | Description | Type | Default | Optional |
|:---------:|-------------|------|---------|:--------:|
| tag   | The unique tag for the field | `NonZeroU32` | N/A | N |
| label (The label is not a valid attribute, valid values are the variants of [`Label`]) | The type specification for the field | [`Label`](#label-attribute) | N/A | N |
| schema    | Defines metadata for the field including its name and descriptive information | [`Schema`](#schema-attribute) | See [`Schema`](#schema-attribute) | Y |
| partial | The options for configuring the field in generated partial object | [`PartialField`](#partial-field-attribute) | See [`PartialField`](#partial-field-attribute) | Y |
| partial_ref | The options for configuring the field in generated partial reference object | [`PartialRefField`](#partial-ref-field-attribute) | See [`PartialRefField`](#partial-ref-field-attribute) | Y |
| ref | The options for configuring the field in generated reference object | [`RefField`](#ref-field-attribute) | See [`RefField`](#ref-field-attribute) | Y |
| selector | The options for configuring the field in generated selector | [`SelectorField`](#selector-field-attribute) | See [`SelectorField`](#selector-field-attribute) | Y |
| encode | The options for configuring how to encode the field | [`Encode`](#encode-field-attribute) | See [`Encode`](#encode-field-attribute) | N/A | Y |
| decode | The options for configuring how to decode the field | [`Decode`](#decode-field-attribute) | See [`Decode`](#decode-field-attribute) | N/A | Y |
| or_default | A bool which specifies if such field is missing when decoding or converting, the default value should be given, mutual exclusive with or_else and ok_or_else, this will overwirte the behavior from the object level or_default settings | `bool` | N/A | Y |
| or_else | A path to a function or closure which specifies if such field is missing when decoding or converting, such function or closure will be invoked, mutual exclusive with or_default and ok_or_else, this will overwirte the behavior from the object level or_default settings | `String \| Closure` | N/A | Y |
| ok_or_else | A path to a function or closure which specifies if such field is missing when decoding or converting, such function or closure will be invoked, mutual exclusive with or_default and or_else, this will overwirte the behavior from the object level or_default settings | `String \| Closure` | N/A | Y |
| copy | The hint if the field is copyable, this configure will overwrite the object level copy attribute settings | `bool` | `false` | Y |


## Label Attribute

Type specification of the field, label is an enum, the possible values are:


| Variant | Description |  Type |
|:---------:|-------------|-----|
| scalar | The type of the field is a scalar | [`ScalarLabel`](#scalar-label-attribute) |
| bytes | The type of the field is a bytes | [`BytesLabel`](#bytes-label-attribute) |
| string | The type of the field is a string | [`StringLabel`](#string-label-attribute) |
| object | The type of the field is an object | [`ObjectLabel`](#object-label-attribute) |
| enum | The type of the field is an enum | [`EnumLabel`](#enum-label-attribute) |
| union | The type of the field is an union | [`UnionLabel`](#union-label-attribute) |
| interface | The type of the field is an interface | [`InterfaceLabel`](#interface-label-attribute) |
| set | The type of the field is a set | [`SetLabel`](#set-label-attribute) |
| map | The type of the field is a map | [`MapLabel`](#map-label-attribute) |
| list | The type of the field is a list | [`ListLabel`](#list-label-attribute) |
| nullable | The type of the field is nullable | [`NullableLabel`](#nullable-label-attribute) |

### Scalar Label Attribute

The scalar type specfication

| Attribute | Description                           | Type     | Default     | Optional |
|:---------:|---------------------------------------|----------|-------------|:--------:|
| as | The specified wire format type for encoding/decoding the field | `String` | the default wire format based on the type | Y |

### Bytes Label Attribute

The bytes type specfication

| Attribute | Description                           | Type     | Default     | Optional |
|:---------:|---------------------------------------|----------|-------------|:--------:|
| as | The specified wire format type for encoding/decoding the field | `String` | the default wire format based on the type | Y |

### String Label Attribute

The bytes type specfication

| Attribute | Description                           | Type     | Default     | Optional |
|:---------:|---------------------------------------|----------|-------------|:--------:|
| as | The specified wire format type for encoding/decoding the field | `String` | the default wire format based on the type | Y |

### Object Label Attribute

The object type specfication

| Attribute | Description                           | Type     | Default     | Optional |
|:---------:|---------------------------------------|----------|-------------|:--------:|
| as | The specified wire format type for encoding/decoding the field | `String` | the default wire format based on the type | Y |

### Enum Label Attribute

The enum type specfication

| Attribute | Description                           | Type     | Default     | Optional |
|:---------:|---------------------------------------|----------|-------------|:--------:|
| as | The specified wire format type for encoding/decoding the field | `String` | the default wire format based on the type | Y |

### Union Label Attribute

The union type specfication

| Attribute | Description                           | Type     | Default     | Optional |
|:---------:|---------------------------------------|----------|-------------|:--------:|
| as | The specified wire format type for encoding/decoding the field | `String` | the default wire format based on the type | Y |

### Interface Label Attribute

The interface type specfication

| Attribute | Description                           | Type     | Default     | Optional |
|:---------:|---------------------------------------|----------|-------------|:--------:|
| as | The specified wire format type for encoding/decoding the field | `String` | the default wire format based on the type | Y |

### Set Label Attribute

The set type specfication

| Attribute | Description                           | Type     | Default     | Optional |
|:---------:|---------------------------------------|----------|-------------|:--------:|
| label | The specification of the inner type | [`Label`](#label-attribute) | N/A | N |
| as | The specified wire format type for encoding/decoding the field | `String` | the default wire format based on the type | Y |

**Note:** The label and as are mutual exclusive, which means you can only set one of them

### Map Label Attribute

The map type specfication

| Attribute | Description                           | Type     | Default     | Optional |
|:---------:|---------------------------------------|----------|-------------|:--------:|
| key | The specification of the key type | [`Label`](#label-attribute) | N/A | N |
| value | The specification of the value type | [`Label`](#label-attribute) | N/A | N |
| as | The specified wire format type for encoding/decoding the field | `String` | the default wire format based on the type | Y |

**Note:** The (key and value) and as are mutual exclusive, which means you can only set either `as` or both `key` and `value`

### List Label Attribute

The list type specfication

| Attribute | Description                           | Type     | Default     | Optional |
|:---------:|---------------------------------------|----------|-------------|:--------:|
| label | The specification of the inner type | [`Label`](#label-attribute) | N/A | N |
| as | The specified wire format type for encoding/decoding the field | `String` | the default wire format based on the type | Y |

**Note:** The label and as are mutual exclusive, which means you can only set one of them

### Nullable Label Attribute

The nullable type specfication

| Attribute | Description                           | Type     | Default     | Optional |
|:---------:|---------------------------------------|----------|-------------|:--------:|
| label | The specification of the inner type | [`Label`](#label-attribute) | N/A | N |
| as | The specified wire format type for encoding/decoding the field | `String` | the default wire format based on the type | Y |

**Note:** The label and as are mutual exclusive, which means you can only set one of them

## Partial Ref Field Attribute

## Partial Field Attribute

## Ref Field Attribute

## Selector Field Attribute

## Encode Field Attribute

## Decode Field Attribute


# stub-it-out

```

v v v Supported Constructs v v v

<variable>: string // variable type

<enum>
  path?: string
  options: array of strings

<struct>
  description: string
  path?: string
  <variable>?
  ...
  static?: 
    <variable>?
    ...

<interface>
  description: string
  path?: string
  method?
    <method>?
    ...
  static?
    method?
      <method>?
      ...

<method>
  description: string
  params?
    <variable>?
    ...
  calls?: array of strings // name of classes, functions, and methods called by the method
  return?: string // data type returned by the method

<function>
  description: string
  path?: string
  params?
    <variable>?
    ...
  calls?: array of strings // name of classes, functions, and methods called by the function
  return?: string // data type returned by the function

<class>
  description: string
  path?: string
  parent?: string // name of the parent class
  private, protected, or public
    <variable>?
    ...
    method?
      <method>?
      ...
    class?
      <class>?
      ...
    enum?
      <enum>?
      ...
    struct?
      <struct>?
      ...
    static?: 
      <variable>?
      ...
      method?
        <method>?
        ...

^ ^ ^ Supported Constructs ^ ^ ^

settings
  name: string // name of the project
  path?: string // path to generated root directory
  otherConfigFiles?: array of strings // paths to other config files from project root

enum?
  <enum>?
  ...

function?:
  <function>?
  ...
  
struct?
  <struct>?
  ...
  
interface?
  <interface>?
  ...
  
class?
  <class>?
  ...

```
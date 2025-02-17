# View persistence

- Generated view is owned by backend
- Widget callbacks accept `view: &mut View<T>` or `this: &mut Widget<T>` (or similar) as well as state (`T`)
  - where these are passed by the backend (which now owns the view)
    - **but how/when to re-layout the view?**
  - Will lifetimes allow this?
- Query the view using Widget id
  - Such as obtaining TextBox text via `get_widget("id")`
    - but how to account for differing types? (want specific fields, etc.)
        - is there a way to use the `::<>` operator as in `get_widget::<T>("id")`?
        - or use `Any` for downcasting `Widget<T>` to specific `Widget` type
  - Use this to update state, modify widgets, etc.
- Hopefully allows animation states
* Create a layout function allowing views to re-layout without the need for reinstantiating each widget
  * Use this function or similar to initialize views to clean up the view macros significantly (their sole purpose should be to receive views/widgets, nothing more)

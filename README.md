# Aeon - A programming language for practical purposes

## Philosophy
Aeon is a better C. Its not aiming to add any fancy new concepts, just fix some annoying
aspects of C.

```aeon

module main

module UserManager {

  pub struct User {
    id: Int,
    name: String,
    email: String,
  }

  // No name means this struct has the same name as the module
  // And inside the module can be referred to as Self
  pub struct {
    users: *User
    count: usize
    capacity: usize
  }

  pub fn new_user(self: *Self, id: Int, name: String, email: String) User {
    user := User {
      id: id,
      name: name,
      email: email,
    }

    index := self.get_next_index()
    self.users[index] = user
    user
  }

  fn get_next_index(self: *Self) usize {
    assert(self.count <= self.capacity);
    if self.count == self.capacity {
      self.capacity += 2
      self.users = realloc(self.users, self.capacity)
    }

    return self.count + 1
  }
}


fn main() void {
  user := User {
    id: 1,
    name: "John Doe",
    email: "john.doe@example.com",
  }

  user_ptr := &user

  printf"User ID: {}\n", user.id)
  printf"User Name: {}\n", user_ptr.name)
  printf"User Email: {}\n", user.email)
}
```

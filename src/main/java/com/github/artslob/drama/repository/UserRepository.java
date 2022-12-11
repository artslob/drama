package com.github.artslob.drama.repository;

import com.github.artslob.drama.entity.User;
import org.springframework.data.repository.CrudRepository;

public interface UserRepository extends CrudRepository<User, String> {}

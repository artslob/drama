package com.github.artslob.drama.entity;

import jakarta.persistence.Entity;
import jakarta.persistence.GeneratedValue;
import jakarta.persistence.GenerationType;
import jakarta.persistence.Id;
import lombok.Data;

@Data
@Entity
public class AccessToken {
    @Id
    @GeneratedValue(strategy= GenerationType.IDENTITY)
    private long id;
    private String accessToken;
    private String tokenType;
    private int expiresIn;
    private String scope;
}

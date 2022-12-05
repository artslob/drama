package com.github.artslob.drama.entity;

import jakarta.persistence.Entity;
import jakarta.persistence.GeneratedValue;
import jakarta.persistence.GenerationType;
import jakarta.persistence.Id;
import lombok.Data;
import lombok.NonNull;

@Data
@Entity
public class AccessToken {
    @Id
    @GeneratedValue(strategy= GenerationType.IDENTITY)
    private long id;
    @NonNull
    private String accessToken;
    @NonNull
    private String tokenType;
    @NonNull
    private int expiresIn;
    @NonNull
    private String scope;
}

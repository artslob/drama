package com.github.artslob.drama.entity;

import jakarta.persistence.Entity;
import jakarta.persistence.GeneratedValue;
import jakarta.persistence.GenerationType;
import jakarta.persistence.Id;
import lombok.Data;
import lombok.NonNull;

@Data
@Entity
public class RefreshToken {
    @Id
    @GeneratedValue(strategy= GenerationType.IDENTITY)
    private long id;
    @NonNull
    private String refreshToken;
    @NonNull
    private String tokenType;
    @NonNull
    private String scope;
}

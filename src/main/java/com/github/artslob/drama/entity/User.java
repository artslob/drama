package com.github.artslob.drama.entity;

import jakarta.persistence.Entity;
import jakarta.persistence.Id;
import jakarta.persistence.Table;
import lombok.*;

@Entity(name = "users")
@Table(name = "users")
@Getter
@Setter
@ToString
@RequiredArgsConstructor
public class User {
    @Id
    private String id;

    private String name;
    private boolean accept_followers;
    private boolean has_subscribed;
    private boolean has_verified_email;
    private boolean hide_from_robots;
    private boolean is_employee;
    private boolean is_gold;
    private boolean is_mod;
    private int total_karma;
    private int link_karma;
    private int awardee_karma;
    private int awarder_karma;
    private int comment_karma;
    private boolean verified;
}

package com.github.artslob.drama.domain;

import com.fasterxml.jackson.annotation.JsonIgnoreProperties;

@JsonIgnoreProperties(ignoreUnknown = true)
public record UserIdentityResponse(
        String id,
        boolean accept_followers,
        boolean has_subscribed,
        boolean has_verified_email,
        boolean hide_from_robots,
        boolean is_employee,
        boolean is_gold,
        boolean is_mod,
        String name,
        int total_karma,
        int link_karma,
        int awardee_karma,
        int awarder_karma,
        int comment_karma,
        boolean verified) {}

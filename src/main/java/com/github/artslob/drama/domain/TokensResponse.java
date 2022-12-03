package com.github.artslob.drama.domain;

import com.fasterxml.jackson.annotation.JsonIgnoreProperties;

@JsonIgnoreProperties(ignoreUnknown = true)
public record TokensResponse(
        String access_token,
        String token_type,
        int expires_in,
        String scope,
        String refresh_token
) {
}

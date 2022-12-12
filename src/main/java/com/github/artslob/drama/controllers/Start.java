package com.github.artslob.drama.controllers;

import com.github.artslob.drama.domain.TokensResponse;
import com.github.artslob.drama.domain.UserIdentityResponse;
import com.github.artslob.drama.entity.AccessToken;
import com.github.artslob.drama.entity.RefreshToken;
import com.github.artslob.drama.entity.User;
import com.github.artslob.drama.properties.MainProperties;
import com.github.artslob.drama.repository.AccessTokenRepository;
import com.github.artslob.drama.repository.RefreshTokenRepository;
import com.github.artslob.drama.repository.UserRepository;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.web.client.RestTemplateBuilder;
import org.springframework.http.HttpEntity;
import org.springframework.http.HttpHeaders;
import org.springframework.http.HttpMethod;
import org.springframework.http.MediaType;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;

@RestController
public class Start {
    @Autowired
    private RestTemplateBuilder restTemplateBuilder;

    @Autowired
    private MainProperties properties;

    @Autowired
    private AccessTokenRepository accessTokenRepository;

    @Autowired
    private RefreshTokenRepository refreshTokenRepository;

    @Autowired
    private UserRepository userRepository;

    @GetMapping("/")
    public String index() {
        var scope = "identity,history,mysubreddits,read";
        var state = "64990aeb-5178-43d3-8ccb-110962843622";
        var url = String.format(
                """
                %s?\
                client_id=%s\
                &response_type=code\
                &state=%s\
                &redirect_uri=%s\
                &duration=permanent\
                &scope=%s\
                """,
                properties.authorize_url, properties.reddit_app_id, state, properties.redirect_uri, scope);
        return String.format("<a href=\"%s\">go here</a>", url);
    }

    @GetMapping("/callback")
    public String callback(
            @RequestParam(required = false) String error,
            @RequestParam(required = false) String code,
            @RequestParam(required = false) String state) {
        // TODO use transactions
        if (error != null) {
            return String.format("error occurred :( : %s", error);
        }
        if (code == null || state == null) {
            return "got empty code or state :(";
        }
        System.out.println(code);
        System.out.println(state);
        var body =
                String.format("grant_type=authorization_code&code=%s&redirect_uri=%s", code, properties.redirect_uri);
        var restTemplates = restTemplateBuilder
                .basicAuthentication(properties.reddit_app_id, properties.reddit_app_password)
                .build();
        var headers = new HttpHeaders();
        headers.setContentType(MediaType.APPLICATION_FORM_URLENCODED);
        headers.add(HttpHeaders.USER_AGENT, "server:com.github.artslob.drama:v0.0.1 (by /u/artslob-api-user)");
        HttpEntity<String> entity = new HttpEntity<>(body, headers);
        var responseEntity = restTemplates.postForEntity(properties.access_token_url, entity, TokensResponse.class);
        // TODO check for 200
        var response = responseEntity.getBody();
        System.out.println(response);
        var token = new AccessToken(
                response.access_token(), response.token_type(), response.expires_in(), response.scope());
        accessTokenRepository.save(token);
        var refreshToken = new RefreshToken(response.refresh_token(), response.token_type(), response.scope());
        refreshTokenRepository.save(refreshToken);
        {
            headers = new HttpHeaders();
            headers.add(HttpHeaders.AUTHORIZATION, String.format("bearer %s", token.getAccessToken()));
            headers.add(HttpHeaders.USER_AGENT, "server:com.github.artslob.drama:v0.0.1 (by /u/artslob-api-user)");
            HttpEntity<String> userRequest = new HttpEntity<>(null, headers);
            restTemplates = restTemplateBuilder.build();
            var userApiUrl = String.format("%s/api/v1/me", properties.api_url);
            var userResponse = restTemplates
                    .exchange(userApiUrl, HttpMethod.GET, userRequest, UserIdentityResponse.class)
                    .getBody();
            System.out.println(userResponse);
            var userEntity = new User();
            userEntity.setId(userResponse.id());
            userEntity.setAccept_followers(userResponse.accept_followers());
            userEntity.setHas_subscribed(userResponse.has_subscribed());
            userEntity.setHas_verified_email(userResponse.has_verified_email());
            userEntity.setHide_from_robots(userResponse.hide_from_robots());
            userEntity.set_employee(userResponse.is_employee());
            userEntity.set_gold(userResponse.is_gold());
            userEntity.set_mod(userResponse.is_mod());
            userEntity.setName(userResponse.name());
            userEntity.setTotal_karma(userResponse.total_karma());
            userEntity.setLink_karma(userResponse.link_karma());
            userEntity.setAwardee_karma(userResponse.awardee_karma());
            userEntity.setAwarder_karma(userResponse.awarder_karma());
            userEntity.setComment_karma(userResponse.comment_karma());
            userEntity.setVerified(userResponse.verified());
            // TODO use create or update
            userRepository.save(userEntity);
        }
        return "success";
    }
}

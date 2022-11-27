package com.github.artslob.drama.controllers;

import com.github.artslob.drama.properties.MainProperties;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.web.client.RestTemplateBuilder;
import org.springframework.http.HttpEntity;
import org.springframework.http.HttpHeaders;
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

    @GetMapping("/")
    public String index() {
        var scope = "identity,history,mysubreddits,read";
        var redirect_uri = "http://localhost:8080/callback";
        var state = "64990aeb-5178-43d3-8ccb-110962843622";
        // TODO remove this application later
        var applicationId = "giud55ItUqIbi591qrFl_A";
        var url = String.format(
                """
                https://www.reddit.com/api/v1/authorize?\
                client_id=%s\
                &response_type=code\
                &state=%s\
                &redirect_uri=%s\
                &duration=permanent\
                &scope=%s\
                """,
                applicationId,
                state,
                redirect_uri,
                scope
        );
        return String.format("<a href=\"%s\">go here</a>", url);
    }

    @GetMapping("/callback")
    public String callback(
            @RequestParam(required = false) String error,
            @RequestParam(required = false) String code,
            @RequestParam(required = false) String state
    ) {
        if (error != null) {
            return String.format("error occurred :( : %s", error);
        }
        if (code == null || state == null) {
            return "got empty code or state :(";
        }
        System.out.println(code);
        System.out.println(state);
        var url = "https://www.reddit.com/api/v1/access_token";
        var redirect_uri = "http://localhost:8080/callback";
        var body = String.format("grant_type=authorization_code&code=%s&redirect_uri=%s", code, redirect_uri);
        var restTemplates = restTemplateBuilder.basicAuthentication(
                properties.reddit_app_id,
                properties.reddit_app_password
        ).build();
        var headers = new HttpHeaders();
        headers.setContentType(MediaType.APPLICATION_FORM_URLENCODED);
        headers.add(HttpHeaders.USER_AGENT, "server:com.github.artslob.drama:v0.0.1 (by /u/artslob-api-user)");
        HttpEntity<String> entity = new HttpEntity<>(body, headers);
        var responseEntity = restTemplates.postForEntity(url, entity, String.class);
        System.out.println(responseEntity.getBody());
        return "success";
    }
}

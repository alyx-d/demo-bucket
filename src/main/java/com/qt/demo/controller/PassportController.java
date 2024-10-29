package com.qt.demo.controller;

import com.qt.demo.common.R;
import com.qt.demo.dto.passport.LoginReqDTO;
import com.qt.demo.dto.passport.LoginSuccessRespDTO;
import jakarta.servlet.http.HttpSession;
import jakarta.validation.Valid;
import jakarta.websocket.Session;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.security.authentication.AuthenticationManager;
import org.springframework.security.authentication.UsernamePasswordAuthenticationToken;
import org.springframework.security.core.context.SecurityContext;
import org.springframework.security.core.context.SecurityContextHolder;
import org.springframework.security.core.userdetails.UsernameNotFoundException;
import org.springframework.security.web.context.HttpSessionSecurityContextRepository;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import java.util.List;

@RestController
@RequestMapping("/passport")
public class PassportController {


    @GetMapping("/test")
    public R<String> test() {
        return R.success("hello");
    }

    @PostMapping("/login")
    public R<LoginSuccessRespDTO> loginSuccessRespDtoR(@Valid LoginReqDTO loginReqDTO, HttpSession session) {
        var username = loginReqDTO.getUsername();
        var password = loginReqDTO.getPassword();
        if (!"qt".equals(username) || !"123456".equals(password)) {
            throw new UsernameNotFoundException("用户名或密码错误");
        }
        SecurityContext context = SecurityContextHolder.createEmptyContext();
        var authentication = UsernamePasswordAuthenticationToken.authenticated(username, password, List.of());
        context.setAuthentication(authentication);
        SecurityContextHolder.setContext(context);
        session.setAttribute(HttpSessionSecurityContextRepository.SPRING_SECURITY_CONTEXT_KEY, context);
        var loginSuccessRespDTO = new LoginSuccessRespDTO();
        loginSuccessRespDTO.setMessage("success");
        return R.success(loginSuccessRespDTO);
    }

    @GetMapping("/logout")
    public R<String> logout(HttpSession session) {
        SecurityContextHolder.clearContext();
        session.removeAttribute(HttpSessionSecurityContextRepository.SPRING_SECURITY_CONTEXT_KEY);
        return R.success("success");
    }

    @GetMapping("/getUserInfo")
    public R<?> getUserInfo() {
        var username = SecurityContextHolder.getContext().getAuthentication().getName();
        return R.success(username);
    }
}

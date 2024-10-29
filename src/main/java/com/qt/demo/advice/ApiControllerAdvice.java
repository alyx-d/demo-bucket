package com.qt.demo.advice;

import com.qt.demo.common.BizCode;
import com.qt.demo.common.R;
import org.springframework.security.core.userdetails.UsernameNotFoundException;
import org.springframework.web.bind.MethodArgumentNotValidException;
import org.springframework.web.bind.annotation.ExceptionHandler;
import org.springframework.web.bind.annotation.RestControllerAdvice;

@RestControllerAdvice
public class ApiControllerAdvice {

    @ExceptionHandler(MethodArgumentNotValidException.class)
    public R<?> methodArgumentNotValidException(MethodArgumentNotValidException e) {
        var message = new StringBuilder();
        e.getBindingResult().getAllErrors().forEach(item -> message.append(item.getDefaultMessage()).append(" "));
        return R.failure(BizCode.ArgumentNotValid, message.toString(), new Object());
    }

    @ExceptionHandler(UsernameNotFoundException.class)
    public R<?> usernameNotFoundException(UsernameNotFoundException e) {
        return R.failure(BizCode.Error, e.getMessage(), new Object());
    }
}

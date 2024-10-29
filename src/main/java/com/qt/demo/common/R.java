package com.qt.demo.common;

import lombok.Getter;
import lombok.Setter;

@Getter
@Setter
public sealed class R<T> {

    private int code;
    private String message;
    private T data;

    public R(int code, String message, T data) {
        this.code = code;
        this.message = message;
        this.data = data;
    }

    public static final class Success<T> extends R<T> {
        public Success(int code, String msg, T data) {
            super(code, msg, data);
        }
    }

    public static final class Failure<T> extends R<T> {
        public Failure(int code, String msg, T data) {
            super(code, msg, data);
        }
    }

    public static R<?> success(BizCode code) {
        return success(code, "", new Object());
    }

    public static <T> R<T> success(T data) {
        return success(BizCode.Ok, "",  data);
    }

    public static R<?> failure(BizCode code) {
        return failure(code, "", new Object());
    }

    public static <T> R<T> success(BizCode code, String message, T data) {
        return new Success<>(code.getCode(), (code.getMessage() + " " + message).trim(), data);
    }
    public static <T> R<T> failure(BizCode code, String message, T data) {
        return new Failure<>(code.getCode(), (code.getMessage() + " " + message).trim(), data);
    }
}

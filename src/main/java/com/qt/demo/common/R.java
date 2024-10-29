package com.qt.demo.common;

public sealed class R<T> {

    private int code;
    private String msg;
    private T data;

    public R(int code, String msg, T data) {
        this.code = code;
        this.msg = msg;
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
}

package user

import (
	"ax123/ecom/config"
	"ax123/ecom/service/auth"
	"ax123/ecom/types"
	"ax123/ecom/utils"
	"fmt"
	"net/http"
	"time"

	"github.com/go-playground/validator/v10" // validator
	"github.com/gorilla/mux"
)

type Handler struct {
	store types.UserStore
}

func NewHandler(store types.UserStore) *Handler {
	return &Handler{store: store}
}

func (h *Handler) RegisterRoutes(router *mux.Router) {
	router.HandleFunc("/login", h.handleLogin).Methods("POST")
	router.HandleFunc("/register", h.handleRegister).Methods("POST")
}

func (h *Handler) handleLogin(w http.ResponseWriter, r *http.Request) {
	//get JSON payload
	var payload types.LoginUserPayLoad
	if err := utils.ParseJSON(r, &payload); err != nil {
		utils.WriteError(w, http.StatusOK, types.HttpResponce{
			Code:    -1,
			Message: err.Error(),
			Data:    nil,
		})
		return
	}
	// validate payload
	if err := utils.Validate.Struct(payload); err != nil {
		utils.WriteError(w, http.StatusOK, types.HttpResponce{
			Code:    -1,
			Message: err.Error(),
			Data:    nil,
		})
		return
	}

	// check if user exists
	u, err := h.store.GetUserByEmail(payload.Email)
	if err != nil {
		utils.WriteError(w, http.StatusOK, types.HttpResponce{
			Code:    -1,
			Message: "invalid email or password",
			Data:    nil,
		})
		return
	}

	if !auth.ComparePassword(payload.Password, u.Password) {
		utils.WriteError(w, http.StatusOK, types.HttpResponce{
			Code:    -1,
			Message: "invalid email or password",
			Data:    nil,
		})
		return
	}
	secret := []byte(config.Envs.JWTSecret)
	token, err := auth.CreateJWT(secret, u.ID)
	if err != nil {
		utils.WriteError(w, http.StatusOK, types.HttpResponce{
			Code:    -1,
			Message: fmt.Sprintf("failed to create JWT token: %s", err.Error()),
			Data:    nil,
		})
		return
	}
	utils.WriteJSON(w, http.StatusOK, types.HttpResponce{
		Code:    0,
		Message: "success",
		Data: map[string]any{
			"token": token,
		},
	})

}

func (h *Handler) handleRegister(w http.ResponseWriter, r *http.Request) {
	//get JSON payload
	var payload types.RegisterUserPayLoad
	if err := utils.ParseJSON(r, &payload); err != nil {
		utils.WriteError(w, http.StatusOK, types.HttpResponce{
			Code:    -1,
			Message: err.Error(),
			Data:    nil,
		})
		return
	}
	// validate payload
	if err := utils.Validate.Struct(payload); err != nil {
		utils.WriteError(w, http.StatusOK, types.HttpResponce{
			Code:    -1,
			Message: err.(validator.ValidationErrors).Error(),
			Data:    nil,
		})
		return
	}
	// check if user exists
	_, err := h.store.GetUserByEmail(payload.Email)
	if err == nil {
		utils.WriteError(w, http.StatusOK, types.HttpResponce{
			Code:    -1,
			Message: fmt.Sprintf("user with email %s already exists", payload.Email),
			Data:    nil,
		})
		return
	}

	// if it doesn't exist, create a new user
	hashedPassword, err := auth.HashPassword(payload.Password)
	if err != nil {
		utils.WriteError(w, http.StatusOK, types.HttpResponce{
			Code:    -1,
			Message: err.Error(),
			Data:    nil,
		})
		return
	}
	u := types.User{
		FirstName: payload.FirstName,
		LastName:  payload.LastName,
		Email:     payload.Email,
		Password:  hashedPassword,
		CreateAt:  time.Now(),
	}
	err = h.store.CreateUser(&u)
	if err != nil {
		utils.WriteError(w, http.StatusOK, types.HttpResponce{
			Code:    -1,
			Message: err.Error(),
			Data:    nil,
		})
		return
	}
	utils.WriteJSON(w, http.StatusOK, types.HttpResponce{
		Code:    0,
		Message: "success",
		Data:    u,
	})

}

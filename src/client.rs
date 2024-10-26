#![cfg(not(doctest))]

use std::env;

use reqwest::{
    header::{self, HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Client, Url,
};
use serde_json::{from_str, Value};

use crate::{
    error::Error::{self, AuthError},
    models::{
        AuthClient, AuthServerHealth, AuthServerSettings, IdTokenCredentials, InviteParams,
        LogoutScope, OAuthResponse, OTPResponse, Provider, RefreshSessionPayload,
        RequestMagicLinkPayload, ResendParams, ResetPasswordForEmailPayload, SendSMSOtpPayload,
        Session, SignInEmailOtpParams, SignInWithEmailAndPasswordPayload,
        SignInWithEmailOtpPayload, SignInWithOAuthOptions, SignInWithPhoneAndPasswordPayload,
        SignInWithSSO, SignUpWithEmailAndPasswordPayload, SignUpWithPasswordOptions,
        SignUpWithPhoneAndPasswordPayload, UpdateUserPayload, User, VerifyOtpParams, AUTH_V1,
    },
};

impl AuthClient {
    /// Create a new Auth Client
    /// You can find your project url and keys at https://supabase.com/dashboard/project/<your project id>/settings/api
    /// # Example
    /// ```
    /// let auth_client = AuthClient::new(project_url, api_key, jwt_secret).unwrap();
    /// ```
    pub fn new(
        project_url: impl Into<String>,
        api_key: impl Into<String>,
        jwt_secret: impl Into<String>,
    ) -> Self {
        AuthClient {
            client: Client::new(),
            project_url: project_url.into(),
            api_key: api_key.into(),
            jwt_secret: jwt_secret.into(),
        }
    }

    /// Create a new AuthClient from environment variables
    /// Requires `SUPABASE_URL`, `SUPABASE_API_KEY`, and `SUPABASE_JWT_SECRET` environment variables
    /// # Example
    /// ```
    /// let auth_client = AuthClient::new_from_env().unwrap();
    ///
    /// assert!(auth_client.project_url == env::var("SUPABASE_URL").unwrap())
    /// ```
    pub fn new_from_env() -> Result<AuthClient, Error> {
        let project_url = env::var("SUPABASE_URL")?;
        let api_key = env::var("SUPABASE_API_KEY")?;
        let jwt_secret = env::var("SUPABASE_JWT_SECRET")?;

        Ok(AuthClient {
            client: Client::new(),
            project_url,
            api_key,
            jwt_secret,
        })
    }

    /// Sign in a user with an email and password
    /// # Example
    /// ```
    /// let session = auth_client
    ///     .login_with_email(demo_email, demo_password)
    ///     .await
    ///     .unwrap();
    ///
    /// assert!(session.user.email == demo_email)
    /// ```
    pub async fn login_with_email<S: Into<String>>(
        &self,
        email: S,
        password: S,
    ) -> Result<Session, Error> {
        let payload = SignInWithEmailAndPasswordPayload {
            email: email.into(),
            password: password.into(),
        };

        let mut headers = header::HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert("apikey", HeaderValue::from_str(&self.api_key)?);
        let body = serde_json::to_string(&payload)?;

        let response = self
            .client
            .post(format!(
                "{}{}/token?grant_type=password",
                self.project_url, AUTH_V1
            ))
            .headers(headers)
            .body(body)
            .send()
            .await?;

        let res_status = response.status();
        let res_body = response.text().await?;

        let session: Session = from_str(&res_body).map_err(|_| AuthError {
            status: res_status,
            message: res_body,
        })?;

        Ok(session)
    }

    /// Sign in a user with phone number and password
    /// # Example
    /// ```
    /// let session = auth_client
    ///     .login_with_phone(demo_phone, demo_password)
    ///     .await
    ///     .unwrap();
    ///
    /// assert!(session.user.phone == demo_phone)
    /// ```
    pub async fn login_with_phone<S: Into<String>>(
        &self,
        phone: S,
        password: S,
    ) -> Result<Session, Error> {
        let payload = SignInWithPhoneAndPasswordPayload {
            phone: phone.into(),
            password: password.into(),
        };

        let mut headers = header::HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json")?);
        headers.insert("apikey", HeaderValue::from_str(&self.api_key)?);

        let body = serde_json::to_string(&payload)?;

        let response = self
            .client
            .post(format!(
                "{}{}/token?grant_type=password",
                self.project_url, AUTH_V1
            ))
            .headers(headers)
            .body(body)
            .send()
            .await?;

        let res_status = response.status();
        let res_body = response.text().await?;

        let session: Session = from_str(&res_body).map_err(|_| AuthError {
            status: res_status,
            message: res_body,
        })?;

        Ok(session)
    }

    /// Sign up a new user with an email and password
    /// # Example
    /// ```
    /// let session = auth_client
    ///     .sign_up_with_email_and_password(demo_email, demo_password)
    ///     .await
    ///     .unwrap();
    ///
    /// assert!(session.user.email == demo_email)
    ///```
    pub async fn sign_up_with_email_and_password<S: Into<String>>(
        &self,
        email: S,
        password: S,
        options: Option<SignUpWithPasswordOptions>,
    ) -> Result<Session, Error> {
        let payload = SignUpWithEmailAndPasswordPayload {
            email: email.into(),
            password: password.into(),
            options,
        };

        let mut headers = header::HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json")?);
        headers.insert("apikey", HeaderValue::from_str(&self.api_key)?);

        let body = serde_json::to_string(&payload)?;

        let response = self
            .client
            .post(format!("{}{}/signup", self.project_url, AUTH_V1))
            .headers(headers)
            .body(body)
            .send()
            .await?;

        let res_status = response.status();
        let res_body = response.text().await?;

        let session: Session = from_str(&res_body).map_err(|_| AuthError {
            status: res_status,
            message: res_body,
        })?;

        Ok(session)
    }

    /// Sign up a new user with an email and password
    /// # Example
    /// ```
    /// let session = auth_client
    ///     .sign_up_with_phone_and_password(demo_phone, demo_password)
    ///     .await
    ///     .unwrap();
    ///
    /// assert!(session.user.phone == demo_phone)
    ///```
    pub async fn sign_up_with_phone_and_password<S: Into<String>>(
        &self,
        phone: S,
        password: S,
        options: Option<SignUpWithPasswordOptions>,
    ) -> Result<Session, Error> {
        let payload = SignUpWithPhoneAndPasswordPayload {
            phone: phone.into(),
            password: password.into(),
            options,
        };

        let mut headers = header::HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json")?);
        headers.insert("apikey", HeaderValue::from_str(&self.api_key)?);

        let body = serde_json::to_string(&payload)?;

        let response = self
            .client
            .post(format!("{}{}/signup", self.project_url, AUTH_V1))
            .headers(headers)
            .body(body)
            .send()
            .await?;

        let res_status = response.status();
        let res_body = response.text().await?;

        let session: Session = from_str(&res_body).map_err(|_| AuthError {
            status: res_status,
            message: res_body,
        })?;

        Ok(session)
    }

    /// Sends a login email containing a magic link
    /// # Example
    /// ```
    /// let _response = auth_client
    ///     .send_login_email_with_magic_link(demo_email)
    ///    .await
    ///    .unwrap();
    ///```
    pub async fn send_login_email_with_magic_link<S: Into<String>>(
        &self,
        email: S,
    ) -> Result<(), Error> {
        let payload = RequestMagicLinkPayload {
            email: email.into(),
        };

        let mut headers = header::HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json")?);
        headers.insert("apikey", HeaderValue::from_str(&self.api_key)?);

        let body = serde_json::to_string(&payload)?;

        let response = self
            .client
            .post(format!("{}{}/magiclink", self.project_url, AUTH_V1))
            .headers(headers)
            .body(body)
            .send()
            .await?;

        let res_status = response.status();
        let res_body = response.text().await?;

        if res_status.is_success() {
            Ok(())
        } else {
            Err(AuthError {
                status: res_status,
                message: res_body,
            })
        }
    }

    // TODO: Fix Return Type https://supabase.com/docs/reference/javascript/auth-signinwithotp

    /// Send a Login OTP via SMS
    ///
    /// # Example
    /// ```
    /// let response = auth_client.send_sms_with_otp(demo_phone).await;
    /// ```
    pub async fn send_sms_with_otp<S: Into<String>>(&self, phone: S) -> Result<OTPResponse, Error> {
        let payload = SendSMSOtpPayload {
            phone: phone.into(),
        };

        let mut headers = header::HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json")?);
        headers.insert("apikey", HeaderValue::from_str(&self.api_key)?);

        let body = serde_json::to_string(&payload)?;

        let response = self
            .client
            .post(format!("{}{}/otp", self.project_url, AUTH_V1))
            .headers(headers)
            .body(body)
            .send()
            .await?;

        let res_status = response.status();
        let res_body = response.text().await?;

        if res_status.is_success() {
            let message = serde_json::from_str(&res_body)?;
            Ok(message)
        } else {
            Err(AuthError {
                status: res_status,
                message: res_body,
            })
        }
    }

    /// Send a Login OTP via email
    ///
    /// Returns an OTPResponse on success
    /// # Example
    /// ```
    /// let send = auth_client.send_sms_with_otp(demo_phone).await.unwrap();
    /// ```
    pub async fn send_email_with_otp<S: Into<String>>(
        &self,
        email: S,
        options: Option<SignInEmailOtpParams>,
    ) -> Result<OTPResponse, Error> {
        let payload = SignInWithEmailOtpPayload {
            email: email.into(),
            options,
        };

        let mut headers = header::HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json")?);
        headers.insert("apikey", HeaderValue::from_str(&self.api_key)?);

        let body = serde_json::to_string(&payload)?;

        let response = self
            .client
            .post(format!("{}{}/otp", self.project_url, AUTH_V1))
            .headers(headers)
            .body(body)
            .send()
            .await?;

        let res_status = response.status();
        let res_body = response.text().await?;

        if res_status.is_success() {
            let message = serde_json::from_str(&res_body)?;
            Ok(message)
        } else {
            Err(AuthError {
                status: res_status,
                message: res_body,
            })
        }
    }

    /// Sign in a user using an OAuth provider.
    /// # Example
    /// ```
    /// // You can add custom parameters using a HashMap<String, String>
    /// let mut params = HashMap::new();
    /// params.insert("key".to_string(), "value".to_string());
    ///
    /// let options = SignInWithOAuthOptions {
    ///     query_params: Some(params),
    ///     redirect_to: Some("localhost".to_string()),
    ///     scopes: Some("repo gist notifications".to_string()),
    ///     skip_brower_redirect: Some(true),
    /// };
    ///
    /// let response = auth_client
    ///     .sign_in_with_oauth(supabase_auth::models::Provider::Github, Some(options))
    ///     .await
    ///     .unwrap();
    /// ```
    pub async fn login_with_oauth(
        &self,
        provider: Provider,
        options: Option<SignInWithOAuthOptions>,
    ) -> Result<OAuthResponse, Error> {
        let mut headers = header::HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json")?);
        headers.insert("apikey", HeaderValue::from_str(&self.api_key)?);

        let body = serde_json::to_string(&options)?;

        let response = self
            .client
            .get(format!(
                "{}{}/authorize?provider={}",
                self.project_url, AUTH_V1, provider
            ))
            .headers(headers)
            .body(body)
            .send()
            .await?;

        let res_status = response.status();
        let url = response.url().to_owned();
        let res_body = response.text().await?;

        if res_status.is_success() {
            Ok(OAuthResponse { url, provider })
        } else {
            Err(AuthError {
                status: res_status,
                message: res_body,
            })
        }
    }

    /// Return the signed in User
    /// # Example
    /// ```
    /// let user = auth_client
    ///     .get_user(session.unwrap().access_token)
    ///     .await
    ///     .unwrap();
    ///
    /// assert!(user.email == demo_email)
    /// ```
    pub async fn get_user<S: Into<String>>(&self, bearer_token: S) -> Result<User, Error> {
        let mut headers = header::HeaderMap::new();
        headers.insert("apikey", HeaderValue::from_str(&self.api_key)?);
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", &bearer_token.into()))?,
        );

        let response = self
            .client
            .get(format!("{}{}/user", self.project_url, AUTH_V1))
            .headers(headers)
            .send()
            .await?;

        let res_status = response.status();
        let res_body = response.text().await?;

        let user: User = serde_json::from_str(&res_body).map_err(|_| AuthError {
            status: res_status,
            message: res_body,
        })?;

        Ok(user)
    }

    /// Update the user, such as changing email or password. Each field (email, password, and data) is optional
    /// # Example
    /// ```
    /// let updated_user_data = UpdateUserPayload {
    ///     email: Some("demo@demo.com".to_string()),
    ///     password: Some("demo_password".to_string()),
    ///     data: None, // This field can hold any valid JSON value
    /// };
    ///
    /// let user = auth_client
    ///     .update_user(updated_user_data, access_token)
    ///     .await
    ///     .unwrap();
    /// ```
    pub async fn update_user<S: Into<String>>(
        &self,
        updated_user: UpdateUserPayload,
        bearer_token: S,
    ) -> Result<User, Error> {
        let mut headers = header::HeaderMap::new();
        headers.insert("apikey", HeaderValue::from_str(&self.api_key)?);
        headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json")?);
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", &bearer_token.into()))?,
        );

        let body = serde_json::to_string::<UpdateUserPayload>(&updated_user)?;

        let response = self
            .client
            .put(format!("{}{}/user", self.project_url, AUTH_V1))
            .headers(headers)
            .body(body)
            .send()
            .await?;

        let res_status = response.status();
        let res_body = response.text().await?;

        let user: User = serde_json::from_str(&res_body).map_err(|_| AuthError {
            status: res_status,
            message: res_body,
        })?;

        Ok(user)
    }

    /// Allows signing in with an OIDC ID token. The authentication provider used should be enabled and configured.
    /// # Example
    /// ```
    /// let credentials = IdTokenCredentials {
    ///     provider: Provider::Github,
    ///     token: "<id-token-from-auth-provider>",
    /// };
    ///
    /// let session = auth_client
    ///     .sign_in_with_id_token(credentials)
    ///     .await
    ///     .unwrap();
    /// ```
    pub async fn sign_in_with_id_token(
        &self,
        credentials: IdTokenCredentials,
    ) -> Result<Session, Error> {
        let mut headers = HeaderMap::new();
        headers.insert("apikey", HeaderValue::from_str(&self.api_key)?);
        headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json")?);

        let body = serde_json::to_string(&credentials)?;

        let response = self
            .client
            .post(format!(
                "{}{}/token?grant_type=id_token",
                self.project_url, AUTH_V1
            ))
            .headers(headers)
            .body(body)
            .send()
            .await?;

        let res_status = response.status();
        let res_body = response.text().await?;

        let session: Session = serde_json::from_str(&res_body).map_err(|_| AuthError {
            status: res_status,
            message: res_body,
        })?;

        Ok(session)
    }

    /// Sends an invite link to an email address.
    /// Requires admin permissions to issue invites
    ///
    /// The data field corresponds to the `raw_user_meta_data` User field
    /// # Example
    /// ```
    /// let demo_email = env::var("DEMO_INVITE").unwrap();
    ///
    /// let user = auth_client
    ///     .invite_user_by_email(&demo_email, None, auth_client.api_key())
    ///     .await
    ///     .unwrap();
    ///```
    pub async fn invite_user_by_email<S: Into<String>>(
        &self,
        email: S,
        data: Option<Value>,
        bearer_token: S,
    ) -> Result<User, Error> {
        let mut headers = HeaderMap::new();
        headers.insert("apikey", HeaderValue::from_str(&self.api_key)?);
        headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json")?);
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", &bearer_token.into()))?,
        );

        let invite_payload = InviteParams {
            email: email.into(),
            data,
        };

        let body = serde_json::to_string(&invite_payload)?;

        let response = self
            .client
            .post(format!("{}{}/invite", self.project_url, AUTH_V1))
            .headers(headers)
            .body(body)
            .send()
            .await?;

        let res_status = response.status();
        let res_body = response.text().await?;

        let user: User = serde_json::from_str(&res_body).map_err(|_| AuthError {
            status: res_status,
            message: res_body,
        })?;

        Ok(user)
    }

    /// Verify the OTP sent to the user
    /// # Example
    /// ```
    /// let params = VerifyEmailOtpParams {
    ///     token: "abc123",
    ///     otp_type: OtpType::EmailChange,
    ///     options: None,
    /// };
    ///
    /// let session = auth_client
    ///     .verify_otp(params)
    ///     .await
    ///     .unwrap();
    ///```
    pub async fn verify_otp(&self, params: VerifyOtpParams) -> Result<Session, Error> {
        let mut headers = HeaderMap::new();
        headers.insert("apikey", HeaderValue::from_str(&self.api_key)?);
        headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json")?);

        let body = serde_json::to_string(&params)?;

        let response = self
            .client
            .post(&format!("{}{}/verify", self.project_url, AUTH_V1))
            .headers(headers)
            .body(body)
            .send()
            .await?;

        let res_status = response.status();
        let res_body = response.text().await?;

        let session: Session = serde_json::from_str(&res_body).map_err(|_| AuthError {
            status: res_status,
            message: res_body,
        })?;

        Ok(session)
    }

    /// Check the Health Status of the Auth Server
    /// # Example
    /// ```
    /// let health = auth_client
    ///     .get_health()
    ///     .await
    ///     .unwrap();
    /// ```
    pub async fn get_health(&self) -> Result<AuthServerHealth, Error> {
        let mut headers = HeaderMap::new();
        headers.insert("apikey", HeaderValue::from_str(&self.api_key)?);

        let response = self
            .client
            .get(&format!("{}{}/health", self.project_url, AUTH_V1))
            .headers(headers)
            .send()
            .await?;

        let res_status = response.status();
        let res_body = response.text().await?;

        let health: AuthServerHealth = serde_json::from_str(&res_body).map_err(|_| AuthError {
            status: res_status,
            message: res_body,
        })?;

        Ok(health)
    }

    /// Retrieve the public settings of the server
    /// # Example
    /// ```
    /// let settings = auth_client
    ///     .get_settings()
    ///     .await
    ///     .unwrap();
    /// ```
    pub async fn get_settings(&self) -> Result<AuthServerSettings, Error> {
        let mut headers = HeaderMap::new();
        headers.insert("apikey", HeaderValue::from_str(&self.api_key)?);

        let response = self
            .client
            .get(&format!("{}{}/settings", self.project_url, AUTH_V1))
            .headers(headers)
            .send()
            .await?;

        let res_status = response.status();
        let res_body = response.text().await?;

        let settings: AuthServerSettings =
            serde_json::from_str(&res_body).map_err(|_| AuthError {
                status: res_status,
                message: res_body,
            })?;

        Ok(settings)
    }

    /// Exchange refresh token for a new session
    /// # Example
    /// ```
    /// // When a user signs in they get a session
    /// let original_session = auth_client
    ///     .sign_in_with_email_and_password(demo_email.as_ref(), demo_password)
    ///     .await
    ///     .unwrap();
    ///
    /// // Exchange the refresh token from the original session to create a new session
    /// let new_session = auth_client
    ///     .refresh_session(original_session.refresh_token)
    ///     .await
    ///     .unwrap();
    /// ```
    pub async fn exchange_token_for_session<S: Into<String>>(
        &self,
        refresh_token: S,
    ) -> Result<Session, Error> {
        let mut headers = HeaderMap::new();
        headers.insert("apikey", HeaderValue::from_str(&self.api_key)?);
        headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json")?);

        let body = serde_json::to_string(&RefreshSessionPayload {
            refresh_token: refresh_token.into(),
        })?;

        let response = self
            .client
            .post(&format!(
                "{}{}/token?grant_type=refresh_token",
                self.project_url, AUTH_V1
            ))
            .headers(headers)
            .body(body)
            .send()
            .await?;

        let res_status = response.status();
        let res_body = response.text().await?;

        let session: Session = from_str(&res_body).map_err(|_| AuthError {
            status: res_status,
            message: res_body,
        })?;

        Ok(session)
    }

    pub async fn refresh_session(&self, refresh_token: String) -> Result<Session, Error> {
        self.exchange_token_for_session(refresh_token).await
    }

    /// Send a password recovery email. Invalid Email addresses will return Error Code 400.
    /// Valid email addresses that are not registered as users will not return an error.
    /// # Example
    /// ```
    /// let response = auth_client.reset_password_for_email(demo_email).await.unwrap();
    /// ```
    pub async fn reset_password_for_email<S: Into<String>>(&self, email: S) -> Result<(), Error> {
        let mut headers = HeaderMap::new();
        headers.insert("apikey", HeaderValue::from_str(&self.api_key)?);
        headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json")?);

        let body = serde_json::to_string(&ResetPasswordForEmailPayload {
            email: email.into(),
        })?;

        let response = self
            .client
            .post(&format!("{}{}/recover", self.project_url, AUTH_V1))
            .headers(headers)
            .body(body)
            .send()
            .await?;

        let res_status = response.status();
        let res_body = response.text().await?;

        if res_status.is_success() {
            Ok(())
        } else {
            Err(Error::AuthError {
                status: res_status,
                message: res_body,
            })
        }
    }

    /// Resends emails for existing signup confirmation, email change, SMS OTP, or phone change OTP.
    /// # Example
    /// ```
    /// // Resend can also take MobileResendParams
    /// let credentials = DesktopResendParams {
    ///     otp_type: supabase_auth::models::EmailOtpType::Email,
    ///     email: demo_email.to_owned(),
    ///     options: None,
    /// };
    ///
    /// let resend = auth_client.resend(ResendParams::Desktop(credentials)).await;
    /// ```
    pub async fn resend(&self, credentials: ResendParams) -> Result<(), Error> {
        let mut headers = HeaderMap::new();
        headers.insert("apikey", HeaderValue::from_str(&self.api_key)?);
        headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json")?);

        let body = serde_json::to_string(&credentials)?;

        let response = self
            .client
            .post(&format!("{}{}/resend", self.project_url, AUTH_V1))
            .headers(headers)
            .body(body)
            .send()
            .await?;

        let res_status = response.status();
        let res_body = response.text().await?;

        if res_status.is_success() {
            Ok(())
        } else {
            Err(Error::AuthError {
                status: res_status,
                message: res_body,
            })
        }
    }

    /// Logs out a user with a given scope
    /// # Example
    /// ```
    /// auth_client.logout(Some(LogoutScope::Global), session.access_token).await.unwrap();
    /// ```
    pub async fn logout<S: Into<String>>(
        &self,
        scope: Option<LogoutScope>,
        bearer_token: S,
    ) -> Result<(), Error> {
        let mut headers = HeaderMap::new();
        headers.insert("apikey", HeaderValue::from_str(&self.api_key)?);
        headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json")?);
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", &bearer_token.into()))?,
        );

        let body = serde_json::to_string(&scope)?;

        let response = self
            .client
            .post(&format!("{}{}/logout", self.project_url, AUTH_V1))
            .headers(headers)
            .body(body)
            .send()
            .await?;

        let res_status = response.status();
        let res_body = response.text().await?;

        if res_status.is_success() {
            Ok(())
        } else {
            Err(Error::AuthError {
                status: res_status,
                message: res_body,
            })
        }
    }

    /// Initiates an SSO Login Flow
    /// Returns the URL where the user must authenticate with the SSO Provider
    ///
    /// WARNING: Requires an SSO Provider and Supabase Pro plan
    ///
    /// # Example
    /// ```
    /// let url = auth_client.sso(params).await.unwrap();
    ///
    /// println!("{}", url.to_string());
    /// ```
    pub async fn sso(&self, params: SignInWithSSO) -> Result<Url, Error> {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert("apikey", HeaderValue::from_str(&self.api_key)?);

        let body = serde_json::to_string::<crate::models::SignInWithSSO>(&params)?;

        let response = self
            .client
            .post(&format!("{}{}/sso", self.project_url, AUTH_V1))
            .headers(headers)
            .body(body)
            .send()
            .await?;

        let res_status = response.status();
        let url = response.url().clone();
        let res_body = response.text().await?;

        if res_status.is_server_error() || res_status.is_client_error() {
            return Err(AuthError {
                status: res_status,
                message: res_body,
            });
        }

        Ok(url)
    }

    /// Get the project URL from an AuthClient
    pub fn project_url(&self) -> &String {
        &self.project_url
    }

    /// Get the API Key from an AuthClient
    pub fn api_key(&self) -> &String {
        &self.api_key
    }

    /// Get the JWT Secret from an AuthClient
    pub fn jwt_secret(&self) -> &String {
        &self.jwt_secret
    }
}

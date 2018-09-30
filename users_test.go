package website

import "testing"

func TestValidateAUser(t *testing.T) {
	user, err := NewUser("Michael", "password1")
	if err != nil {
		t.Fatal(err)
	}

	if user.Name != "Michael" {
		t.Errorf("Expected name to be \"Michael\", found %s", user.Name)
	}

	if !user.PasswordIsValid("password1") {
		t.Error("Valid password should be valid")
	}

	if user.PasswordIsValid("invalid password") {
		t.Error("Invalid password should be invalid")
	}
}

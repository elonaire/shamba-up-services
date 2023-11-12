-- A schema-full user table.
DEFINE TABLE user SCHEMAFULL;

-- define some fields.
DEFINE FIELD user_name ON TABLE user TYPE string;
DEFINE INDEX userNameIndex ON TABLE user COLUMNS user_name UNIQUE;
DEFINE FIELD first_name ON TABLE user TYPE string;
DEFINE FIELD middle_name ON TABLE user TYPE string;
DEFINE FIELD last_name ON TABLE user TYPE string;
DEFINE FIELD gender ON TABLE user TYPE string
-- Allow only these values in the array
  ASSERT $value INSIDE ["Male", "Female", "Other"];
DEFINE FIELD dob ON TABLE user TYPE string;
DEFINE FIELD email ON TABLE user TYPE string
  ASSERT string::is::email($value);
DEFINE INDEX userEmailIndex ON TABLE user COLUMNS email UNIQUE;
DEFINE FIELD country ON TABLE user TYPE string;
DEFINE FIELD phone ON TABLE user TYPE string
  ASSERT string::len($value) == 10;
DEFINE INDEX userPhoneIndex ON TABLE user COLUMNS phone UNIQUE;
DEFINE FIELD password ON TABLE user TYPE string;
DEFINE FIELD created_at ON TABLE user TYPE string;
DEFINE FIELD updated_at ON TABLE user TYPE string;
DEFINE FIELD roles ON TABLE user TYPE array
  DEFAULT RETURN fn::get_default_role();
DEFINE FIELD roles.* ON TABLE user TYPE record();
DEFINE FIELD status ON TABLE user TYPE string
  DEFAULT "Active"
-- Allow only these values in the array
  ASSERT $value INSIDE ["Active", "Inactive", "Suspended", "Deleted"];
DEFINE FIELD oauth_client ON TABLE user TYPE option<string>
-- Allow only these values in the array
  ASSERT $value INSIDE ["Google", "Github", None];


-- A schema-full role table.
DEFINE TABLE role SCHEMAFULL;
-- define some fields.
DEFINE FIELD role_name ON TABLE role TYPE string
-- Allow only these values in the array
  ASSERT $value INSIDE ["Admin", "User", "Guest"];
DEFINE INDEX roleNameIndex ON TABLE role COLUMNS role_name UNIQUE;

DEFINE FUNCTION fn::get_default_role() {
  LET $result = SELECT * FROM role WHERE role_name = "User" LIMIT 1;
  RETURN $result.id;
};

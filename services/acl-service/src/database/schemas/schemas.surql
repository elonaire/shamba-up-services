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
-- DEFINE FIELD roles ON TABLE user TYPE array
--   DEFAULT RETURN fn::get_default_role();
-- DEFINE FIELD roles.* ON TABLE user TYPE record<role>;
DEFINE EVENT user_created ON TABLE user WHEN $event = "CREATE" THEN (
    -- add default role to user
    fn::assign_default_role($after.id)
);
DEFINE FIELD status ON TABLE user TYPE string
  DEFAULT "Inactive"
-- Allow only these values in the array
  ASSERT $value INSIDE ["Active", "Inactive", "Suspended", "Deleted"];
DEFINE FIELD oauth_client ON TABLE user TYPE option<string>
-- Allow only these values in the array
  ASSERT $value INSIDE ["Google", "Github", None];
DEFINE FIELD address ON TABLE user TYPE option<string>
  DEFAULT None;
DEFINE FIELD profile_picture ON TABLE user TYPE option<string>
  DEFAULT None;
DEFINE FIELD bio ON TABLE user TYPE option<string>
  DEFAULT None;
DEFINE FIELD website ON TABLE user TYPE option<string>
  DEFAULT None;
DEFINE FIELD professional_details ON TABLE user TYPE option<record<professional_details>>
  DEFAULT None;
DEFINE FIELD cv_link ON TABLE user TYPE option<string>
  DEFAULT None;

-- A schema-full role table.
DEFINE TABLE role SCHEMAFULL;
-- define some fields.
DEFINE FIELD role_name ON TABLE role TYPE string
-- Allow only these values in the array
  ASSERT $value INSIDE ["Admin", "User", "Guest"];
DEFINE INDEX roleNameIndex ON TABLE role COLUMNS role_name UNIQUE;

-- DEFINE FUNCTION fn::get_default_role() {
--   LET $result = SELECT * FROM role WHERE role_name = "User" LIMIT 1;
--   RETURN $result.id;
-- };

-- Function to check if user has a role, if not, assign a default role using edge relationship
DEFINE FUNCTION fn::assign_default_role($user_id: record) {
  LET $results = SELECT ->has_role FROM $user_id;
  LET $edge_name = "->has_role";

  -- LET $results = SELECT VALUE id FROM type::table("has_role") WHERE in = $user_id;
  if array::len($results[0][$edge_name]) == 0 {
    LET $default_role = SELECT * FROM role WHERE role_name = "User" LIMIT 1;
    LET $role_id = $default_role[0].id;
    RELATE $user_id->has_role->$role_id CONTENT {
      in: $user_id,
      out: $role_id
    };
  };
};

-- A schema-full professional_details table.
DEFINE TABLE professional_details SCHEMAFULL;
-- define some fields.
DEFINE FIELD occupation ON TABLE professional_details TYPE string;
DEFINE FIELD description ON TABLE professional_details TYPE string;
DEFINE FIELD start_date ON TABLE professional_details TYPE string;

-- A schema-full portfolio table.
DEFINE TABLE portfolio SCHEMAFULL;
-- define some fields.
DEFINE FIELD title ON TABLE portfolio TYPE string;
DEFINE FIELD description ON TABLE portfolio TYPE string;
DEFINE FIELD start_date ON TABLE portfolio TYPE string;
DEFINE FIELD end_date ON TABLE portfolio TYPE string;
DEFINE FIELD link ON TABLE portfolio TYPE string;
DEFINE FIELD image ON TABLE portfolio TYPE string;
DEFINE FIELD category ON TABLE portfolio TYPE string
-- Allow only these values in the array
  ASSERT $value INSIDE ["JavaScript", "Rust", "Databases", "Cloud", "DevOps"];

-- Relationship between user and roles
DEFINE TABLE has_role SCHEMAFULL;
DEFINE INDEX userRoleIndex ON TABLE has_role 
COLUMNS in, out UNIQUE;
DEFINE FIELD in ON TABLE has_role TYPE record<user>;
DEFINE FIELD out ON TABLE has_role TYPE record<role>;

-- Relationship between user and portfolio
DEFINE TABLE has_portfolio SCHEMAFULL;
DEFINE INDEX userPortfolioIndex ON TABLE has_portfolio
COLUMNS in, out UNIQUE;
DEFINE FIELD in ON TABLE has_portfolio TYPE record<user>;
DEFINE FIELD out ON TABLE has_portfolio TYPE record<portfolio>;

-- A schema-full resume table.
DEFINE TABLE resume SCHEMAFULL;
-- define some fields.
DEFINE FIELD section ON TABLE resume TYPE string;
DEFINE FIELD title ON TABLE resume TYPE string;
DEFINE FIELD description ON TABLE resume TYPE string;
DEFINE FIELD start_date ON TABLE resume TYPE string;
DEFINE FIELD end_date ON TABLE resume TYPE option<string>
  DEFAULT None;
DEFINE FIELD link ON TABLE resume TYPE option<string>
  DEFAULT None;

-- A schema-full achievement table. Should relate to resume table
DEFINE TABLE achievement SCHEMAFULL;
-- define some fields.
DEFINE FIELD description ON TABLE achievement TYPE string;

-- Relationship between resume and achievement
DEFINE TABLE has_achievement SCHEMAFULL;
DEFINE INDEX resumeAchievementIndex ON TABLE has_achievement
COLUMNS in, out UNIQUE;
DEFINE FIELD in ON TABLE has_achievement TYPE record<resume>;
DEFINE FIELD out ON TABLE has_achievement TYPE record<achievement>;

-- Relationship between user and resume
DEFINE TABLE has_resume SCHEMAFULL;
DEFINE INDEX userResumeIndex ON TABLE has_resume
COLUMNS in, out UNIQUE;
DEFINE FIELD in ON TABLE has_resume TYPE record<user>;
DEFINE FIELD out ON TABLE has_resume TYPE record<resume>;

-- Aschema-full service table.
DEFINE TABLE service SCHEMAFULL;
-- define some fields.
DEFINE FIELD title ON TABLE service TYPE string;
DEFINE FIELD description ON TABLE service TYPE string;
DEFINE FIELD image ON TABLE service TYPE string;

-- Relationship between user and service
DEFINE TABLE offers_service SCHEMAFULL;
DEFINE INDEX userServiceIndex ON TABLE offers_service
COLUMNS in, out UNIQUE;
DEFINE FIELD in ON TABLE offers_service TYPE record<user>;
DEFINE FIELD out ON TABLE offers_service TYPE record<service>;

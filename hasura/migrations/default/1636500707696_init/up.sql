SET check_function_bodies = false;

CREATE TABLE "public"."product_test" ("id" integer NOT NULL, "name" text NOT NULL, PRIMARY KEY ("id") );
CREATE TABLE "public"."test_time" ("id" serial NOT NULL, "time" timestamptz NOT NULL DEFAULT now(), PRIMARY KEY ("id") );

.PHONY: up down cleanup

up:
	$(MAKE) -C backend/rust-kratos infra-up
	$(MAKE) -C backend/rust-kratos up
	$(MAKE) -C frontend up

down:
	$(MAKE) -C backend/rust-kratos infra-down
	$(MAKE) -C backend/rust-kratos down
	$(MAKE) -C frontend down

cleanup:
	$(MAKE) -C infrastructure/local_development cleanup

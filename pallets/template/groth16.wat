(module
  (type (;0;) (func (param i32 i32)))
  (type (;1;) (func (param i32 i32 i32)))
  (type (;2;) (func (param i32)))
  (type (;3;) (func (param i32 i32 i32 i32)))
  (type (;4;) (func (param i32 i32 i32) (result i32)))
  (type (;5;) (func (param i32 i32) (result i32)))
  (type (;6;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;7;) (func (result i32)))
  (type (;8;) (func (param i32) (result i32)))
  (type (;9;) (func (param i32 i32 i32 i32 i32)))
  (type (;10;) (func (param i32 i32 i32 i32 i32 i32)))
  (type (;11;) (func (param i32 i32 i32 i32 i32) (result i32)))
  (type (;12;) (func (param i32) (result i64)))
  (import "seal0" "seal_value_transferred" (func (;0;) (type 0)))
  (import "seal0" "seal_input" (func (;1;) (type 0)))
  (import "seal0" "seal_return" (func (;2;) (type 1)))
  (import "seal0" "seal_call_chain_extension" (func (;3;) (type 11)))
  (import "env" "memory" (memory (;0;) 2 16))
  (func (;4;) (type 2) (param i32)
    local.get 0
    call 5
    local.get 0
    call 6)
  (func (;5;) (type 2) (param i32)
    (local i32)
    local.get 0
    i32.load offset=8
    i32.const 12
    i32.mul
    local.set 1
    local.get 0
    i32.load
    local.set 0
    loop  ;; label = @1
      local.get 1
      if  ;; label = @2
        local.get 1
        i32.const -12
        i32.add
        local.set 1
        local.get 0
        call 26
        local.get 0
        i32.const 12
        i32.add
        local.set 0
        br 1 (;@1;)
      end
    end)
  (func (;6;) (type 2) (param i32)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    local.get 0
    call 42
    block  ;; label = @1
      local.get 1
      i32.load
      local.tee 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.load offset=4
      local.tee 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      local.get 2
      call 8
    end
    local.get 1
    i32.const 16
    i32.add
    global.set 0)
  (func (;7;) (type 2) (param i32)
    (local i32 i32)
    block  ;; label = @1
      local.get 0
      i32.load8_u
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.const 4
      i32.add
      i32.load8_u
      br_if 0 (;@1;)
      local.get 0
      i32.const 8
      i32.add
      i32.load8_u
      local.tee 1
      i32.const 7
      i32.and
      i32.const 4
      i32.le_u
      i32.const 0
      local.get 1
      i32.const 1
      i32.ne
      select
      br_if 0 (;@1;)
      local.get 0
      i32.const 12
      i32.add
      i32.load
      local.tee 1
      i32.load
      local.get 1
      i32.load offset=4
      i32.load
      call_indirect (type 2)
      local.get 1
      i32.load offset=4
      i32.load offset=4
      local.tee 2
      if  ;; label = @2
        local.get 1
        i32.load
        local.get 2
        call 8
      end
      local.get 0
      i32.load offset=12
      i32.const 12
      call 8
    end)
  (func (;8;) (type 0) (param i32 i32)
    local.get 0
    local.get 1
    call 37)
  (func (;9;) (type 12) (param i32) (result i64)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    i64.const 0
    i64.store8 offset=12
    local.get 1
    i64.const 0
    i64.store32 offset=8
    block (result i32)  ;; label = @1
      loop  ;; label = @2
        local.get 2
        i32.const 4
        i32.eq
        if  ;; label = @3
          local.get 1
          i64.load8_u offset=12
          i32.wrap_i64
          i32.const 4
          i32.lt_u
          local.set 2
          local.get 1
          i32.load offset=8
          br 2 (;@1;)
        end
        local.get 1
        local.get 0
        call 10
        local.get 1
        i32.load8_u
        i32.const 1
        i32.and
        i32.eqz
        if  ;; label = @3
          local.get 1
          i32.const 8
          i32.add
          local.get 2
          i32.add
          local.get 1
          i32.load8_u offset=1
          i32.store8
          local.get 1
          local.get 2
          i32.const 1
          i32.add
          local.tee 2
          i32.store8 offset=12
          br 1 (;@2;)
        end
      end
      local.get 2
      i32.const 255
      i32.and
      if  ;; label = @2
        local.get 1
        i32.const 0
        i32.store8 offset=12
      end
      i32.const 1
      local.set 2
      i32.const 0
    end
    local.set 0
    local.get 1
    i32.const 16
    i32.add
    global.set 0
    local.get 2
    i64.extend_i32_u
    local.get 0
    i64.extend_i32_u
    i64.const 8
    i64.shl
    i64.or)
  (func (;10;) (type 0) (param i32 i32)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 0
    i32.store8 offset=15
    block  ;; label = @1
      local.get 1
      local.get 2
      i32.const 15
      i32.add
      i32.const 1
      call 24
      i32.eqz
      if  ;; label = @2
        local.get 2
        i32.load8_u offset=15
        local.set 1
        br 1 (;@1;)
      end
      i32.const 1
      local.set 3
    end
    local.get 0
    local.get 1
    i32.store8 offset=1
    local.get 0
    local.get 3
    i32.store8
    local.get 2
    i32.const 16
    i32.add
    global.set 0)
  (func (;11;) (type 0) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i64 i64)
    global.get 0
    i32.const -64
    i32.add
    local.tee 2
    global.set 0
    local.get 2
    i32.const 8
    i32.add
    local.get 1
    call 12
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.load offset=8
        if  ;; label = @3
          local.get 0
          i32.const 0
          i32.store
          br 1 (;@2;)
        end
        block  ;; label = @3
          block  ;; label = @4
            local.get 2
            i32.load offset=12
            local.tee 8
            local.get 1
            i32.load offset=4
            i32.const 12
            i32.div_u
            local.tee 3
            local.get 3
            local.get 8
            i32.gt_u
            select
            i64.extend_i32_u
            i64.const 12
            i64.mul
            local.tee 11
            i64.const 32
            i64.shr_u
            i32.wrap_i64
            br_if 0 (;@4;)
            local.get 11
            i32.wrap_i64
            local.tee 3
            i32.const -1
            i32.le_s
            br_if 0 (;@4;)
            i32.const 4
            local.set 5
            local.get 3
            if  ;; label = @5
              local.get 3
              local.tee 6
              i32.const 4
              call 13
              local.tee 5
              i32.eqz
              br_if 4 (;@1;)
            end
            local.get 2
            i32.const 0
            i32.store offset=24
            local.get 2
            local.get 5
            i32.store offset=16
            local.get 2
            local.get 6
            i32.const 12
            i32.div_u
            local.tee 3
            i32.store offset=20
            i32.const 4
            local.set 6
            block  ;; label = @5
              block  ;; label = @6
                loop  ;; label = @7
                  local.get 7
                  local.get 8
                  i32.eq
                  if  ;; label = @8
                    local.get 0
                    i32.const 8
                    i32.add
                    local.get 8
                    i32.store
                    local.get 2
                    local.get 3
                    i32.store offset=20
                    local.get 0
                    local.get 2
                    i64.load offset=16
                    i64.store align=4
                    br 6 (;@2;)
                  end
                  local.get 2
                  i32.const 48
                  i32.add
                  local.get 1
                  call 14
                  local.get 2
                  i32.load offset=48
                  local.tee 10
                  i32.eqz
                  br_if 4 (;@3;)
                  local.get 2
                  i64.load offset=52 align=4
                  local.set 11
                  block  ;; label = @8
                    local.get 3
                    local.get 7
                    i32.eq
                    if  ;; label = @9
                      local.get 3
                      i32.const 1
                      i32.add
                      local.tee 4
                      local.get 3
                      i32.lt_u
                      br_if 3 (;@6;)
                      local.get 3
                      local.get 3
                      i32.add
                      local.tee 9
                      local.get 3
                      i32.lt_u
                      br_if 5 (;@4;)
                      block (result i32)  ;; label = @10
                        local.get 3
                        if  ;; label = @11
                          local.get 3
                          i64.extend_i32_u
                          i64.const 12
                          i64.mul
                          local.tee 12
                          i64.const 32
                          i64.shr_u
                          i32.wrap_i64
                          br_if 7 (;@4;)
                          local.get 2
                          i32.const 4
                          i32.store offset=56
                          local.get 2
                          local.get 12
                          i64.store32 offset=52
                          local.get 2
                          local.get 5
                          i32.store offset=48
                          local.get 3
                          br 1 (;@10;)
                        end
                        local.get 2
                        i32.const 0
                        i32.store offset=48
                        i32.const 0
                      end
                      local.set 5
                      local.get 2
                      i32.const 32
                      i32.add
                      local.get 9
                      local.get 4
                      local.get 9
                      local.get 4
                      i32.gt_u
                      select
                      local.tee 4
                      i32.const 4
                      local.get 4
                      i32.const 4
                      i32.gt_u
                      select
                      i64.extend_i32_u
                      i64.const 12
                      i64.mul
                      local.tee 12
                      i32.wrap_i64
                      local.get 12
                      i64.const 32
                      i64.shr_u
                      i64.eqz
                      i32.const 2
                      i32.shl
                      local.get 2
                      i32.const 48
                      i32.add
                      call 15
                      local.get 2
                      i32.load offset=40
                      local.set 4
                      local.get 2
                      i32.load offset=32
                      i32.const 1
                      i32.eq
                      br_if 1 (;@8;)
                      local.get 2
                      local.get 2
                      i32.load offset=36
                      local.tee 5
                      i32.store offset=16
                      local.get 4
                      i32.const 12
                      i32.div_u
                      local.set 3
                    end
                    local.get 5
                    local.get 6
                    i32.add
                    local.tee 4
                    local.get 11
                    i64.store align=4
                    local.get 4
                    i32.const -4
                    i32.add
                    local.get 10
                    i32.store
                    local.get 7
                    i32.const -1
                    i32.eq
                    br_if 4 (;@4;)
                    local.get 7
                    i32.const 1
                    i32.add
                    local.set 7
                    local.get 6
                    i32.const 12
                    i32.add
                    local.set 6
                    br 1 (;@7;)
                  end
                end
                local.get 2
                local.get 5
                i32.store offset=20
                local.get 2
                local.get 3
                i32.store offset=24
                br 1 (;@5;)
              end
              local.get 2
              local.get 3
              i32.store offset=20
              local.get 2
              local.get 3
              i32.store offset=24
              i32.const 0
              local.set 4
            end
            local.get 4
            br_if 3 (;@1;)
          end
          unreachable
        end
        local.get 0
        i32.const 0
        i32.store
        local.get 2
        local.get 3
        i32.store offset=20
        local.get 2
        local.get 7
        i32.store offset=24
        local.get 2
        i32.const 16
        i32.add
        call 4
      end
      local.get 2
      i32.const -64
      i32.sub
      global.set 0
      return
    end
    unreachable)
  (func (;12;) (type 0) (param i32 i32)
    (local i32 i32 i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 8
    i32.add
    local.get 1
    call 10
    i32.const 1
    local.set 3
    block  ;; label = @1
      local.get 2
      i32.load8_u offset=8
      i32.const 1
      i32.and
      br_if 0 (;@1;)
      local.get 2
      i32.load8_u offset=9
      local.tee 5
      i32.const 3
      i32.and
      local.tee 4
      i32.const 3
      i32.ne
      if  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 4
              i32.const 1
              i32.sub
              br_table 1 (;@4;) 2 (;@3;) 0 (;@5;)
            end
            local.get 5
            i32.const 252
            i32.and
            i32.const 2
            i32.shr_u
            local.set 4
            i32.const 0
            local.set 3
            br 3 (;@1;)
          end
          local.get 2
          local.get 5
          i32.store8 offset=21
          local.get 2
          i32.const 1
          i32.store8 offset=20
          local.get 2
          local.get 1
          i32.store offset=16
          local.get 2
          i32.const 0
          i32.store16 offset=28
          local.get 2
          i32.const 16
          i32.add
          local.get 2
          i32.const 28
          i32.add
          i32.const 2
          call 74
          br_if 2 (;@1;)
          local.get 2
          i32.load16_u offset=28
          local.tee 1
          i32.const 255
          i32.le_u
          br_if 2 (;@1;)
          local.get 1
          i32.const 2
          i32.shr_u
          local.set 4
          i32.const 0
          local.set 3
          br 2 (;@1;)
        end
        local.get 2
        local.get 5
        i32.store8 offset=21
        local.get 2
        i32.const 1
        i32.store8 offset=20
        local.get 2
        local.get 1
        i32.store offset=16
        local.get 2
        i32.const 0
        i32.store offset=28
        local.get 2
        i32.const 16
        i32.add
        local.get 2
        i32.const 28
        i32.add
        i32.const 4
        call 74
        br_if 1 (;@1;)
        local.get 2
        i32.load offset=28
        local.tee 1
        i32.const 65536
        i32.lt_u
        local.set 3
        local.get 1
        i32.const 2
        i32.shr_u
        local.set 4
        br 1 (;@1;)
      end
      local.get 5
      i32.const 3
      i32.gt_u
      br_if 0 (;@1;)
      i32.const 0
      local.set 3
      local.get 2
      i32.const 0
      i32.store offset=16
      block  ;; label = @2
        local.get 1
        local.get 2
        i32.const 16
        i32.add
        i32.const 4
        call 24
        i32.eqz
        if  ;; label = @3
          local.get 2
          i32.load offset=16
          local.set 4
          br 1 (;@2;)
        end
        i32.const 1
        local.set 3
      end
      local.get 3
      local.get 4
      i32.const 1073741824
      i32.lt_u
      i32.or
      local.set 3
    end
    local.get 0
    local.get 4
    i32.store offset=4
    local.get 0
    local.get 3
    i32.store
    local.get 2
    i32.const 32
    i32.add
    global.set 0)
  (func (;13;) (type 5) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call 36)
  (func (;14;) (type 0) (param i32 i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 8
    i32.add
    local.get 1
    call 12
    block  ;; label = @1
      local.get 2
      i32.load offset=8
      if  ;; label = @2
        local.get 0
        i32.const 0
        i32.store
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 1
        i32.load offset=4
        local.get 2
        i32.load offset=12
        local.tee 3
        i32.lt_u
        br_if 0 (;@2;)
        local.get 2
        local.get 3
        i32.const 1
        call 35
        local.get 2
        local.get 3
        i32.store offset=24
        local.get 2
        local.get 2
        i32.load offset=4
        i32.store offset=20
        local.get 2
        local.get 2
        i32.load
        local.tee 4
        i32.store offset=16
        local.get 1
        local.get 4
        local.get 3
        call 24
        if  ;; label = @3
          local.get 2
          i32.const 16
          i32.add
          call 26
          br 1 (;@2;)
        end
        local.get 4
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        local.get 2
        i64.load offset=20 align=4
        i64.store offset=4 align=4
        local.get 0
        local.get 4
        i32.store
        br 1 (;@1;)
      end
      local.get 0
      i32.const 0
      i32.store
    end
    local.get 2
    i32.const 32
    i32.add
    global.set 0)
  (func (;15;) (type 3) (param i32 i32 i32 i32)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 4
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        if  ;; label = @3
          local.get 1
          i32.const 0
          i32.lt_s
          br_if 1 (;@2;)
          block (result i32)  ;; label = @4
            local.get 3
            i32.load
            local.tee 5
            i32.eqz
            if  ;; label = @5
              local.get 4
              local.get 1
              local.get 2
              call 33
              local.get 4
              i32.load
              local.set 3
              local.get 4
              i32.load offset=4
              br 1 (;@4;)
            end
            local.get 3
            i32.load offset=4
            local.tee 3
            i32.eqz
            if  ;; label = @5
              local.get 4
              i32.const 8
              i32.add
              local.get 1
              local.get 2
              i32.const 0
              call 34
              local.get 4
              i32.load offset=8
              local.set 3
              local.get 4
              i32.load offset=12
              br 1 (;@4;)
            end
            local.get 5
            local.get 3
            local.get 2
            local.get 1
            call 28
            local.set 3
            local.get 1
          end
          local.set 5
          local.get 0
          block (result i32)  ;; label = @4
            local.get 3
            i32.eqz
            if  ;; label = @5
              local.get 0
              local.get 1
              i32.store offset=4
              i32.const 1
              br 1 (;@4;)
            end
            local.get 0
            local.get 3
            i32.store offset=4
            local.get 5
            local.set 2
            i32.const 0
          end
          i32.store
          local.get 0
          i32.const 8
          i32.add
          local.get 2
          i32.store
          br 2 (;@1;)
        end
        local.get 0
        local.get 1
        i32.store offset=4
        local.get 0
        i32.const 1
        i32.store
        local.get 0
        i32.const 8
        i32.add
        i32.const 0
        i32.store
        br 1 (;@1;)
      end
      local.get 0
      i32.const 1
      i32.store
      local.get 0
      i32.const 8
      i32.add
      i32.const 0
      i32.store
    end
    local.get 4
    i32.const 16
    i32.add
    global.set 0)
  (func (;16;) (type 2) (param i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    local.get 0
    i32.store offset=12
    local.get 1
    i32.const 12
    i32.add
    i32.load
    i32.load8_u
    call 17
    unreachable)
  (func (;17;) (type 2) (param i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    i32.const 24
    i32.add
    i32.const 16384
    i32.store
    local.get 1
    i32.const 65672
    i32.store offset=20
    local.get 1
    i32.const 0
    i32.store offset=16
    local.get 1
    i32.const 8
    i32.add
    local.get 1
    i32.const 16
    i32.add
    local.get 0
    call 18
    i32.const 0
    local.get 1
    i32.load offset=8
    local.get 1
    i32.load offset=12
    call 2
    unreachable)
  (func (;18;) (type 1) (param i32 i32 i32)
    (local i32 i32 i32 i64)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    global.set 0
    local.get 1
    i64.load offset=4 align=4
    local.set 6
    local.get 1
    i32.const 8
    i32.add
    local.tee 4
    i32.const 0
    i32.store
    local.get 1
    i32.const 65572
    i32.store offset=4
    local.get 3
    i32.const 0
    i32.store offset=8
    local.get 3
    local.get 6
    i64.store
    local.get 3
    local.get 2
    i32.store8 offset=15
    local.get 3
    local.get 3
    i32.const 15
    i32.add
    i32.const 1
    call 19
    local.get 4
    i32.const 0
    i32.store
    local.get 1
    i32.const 65572
    i32.store offset=4
    local.get 3
    i32.load offset=4
    local.tee 5
    local.get 3
    i32.load offset=8
    local.tee 2
    i32.lt_u
    if  ;; label = @1
      unreachable
    end
    local.get 3
    i32.load
    local.set 4
    local.get 1
    local.get 5
    local.get 2
    i32.sub
    i32.store offset=8
    local.get 1
    local.get 2
    local.get 4
    i32.add
    i32.store offset=4
    local.get 0
    local.get 2
    i32.store offset=4
    local.get 0
    local.get 4
    i32.store
    local.get 3
    i32.const 16
    i32.add
    global.set 0)
  (func (;19;) (type 1) (param i32 i32 i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load offset=8
        local.tee 4
        local.get 2
        i32.add
        local.tee 5
        local.get 4
        i32.ge_u
        if  ;; label = @3
          local.get 3
          i32.const 8
          i32.add
          local.get 4
          local.get 5
          local.get 0
          i32.load
          local.get 0
          i32.load offset=4
          call 38
          local.get 3
          i32.load offset=12
          local.get 2
          i32.ne
          br_if 1 (;@2;)
          local.get 3
          i32.load offset=8
          local.get 1
          local.get 2
          call 81
          drop
          local.get 0
          i32.load offset=8
          local.tee 1
          local.get 2
          i32.add
          local.tee 2
          local.get 1
          i32.ge_u
          br_if 2 (;@1;)
        end
        unreachable
      end
      unreachable
    end
    local.get 0
    local.get 2
    i32.store offset=8
    local.get 3
    i32.const 16
    i32.add
    global.set 0)
  (func (;20;) (type 7) (result i32)
    i32.const 0
    call 21
    i32.const 255
    i32.and
    i32.const 2
    i32.shl
    i32.const 65536
    i32.add
    i32.load)
  (func (;21;) (type 8) (param i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i64)
    global.get 0
    i32.const 288
    i32.sub
    local.tee 1
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 0
                        if  ;; label = @11
                          local.get 1
                          i32.const 16384
                          i32.store offset=20
                          local.get 1
                          i32.const 65672
                          i32.store offset=16
                          local.get 1
                          i32.const 16
                          i32.add
                          call 25
                          local.get 1
                          local.get 1
                          i64.load offset=16
                          i64.store offset=32
                          i32.const 1
                          local.set 0
                          local.get 1
                          i32.const 32
                          i32.add
                          call 9
                          local.tee 9
                          i32.wrap_i64
                          local.tee 2
                          i32.const 1
                          i32.and
                          br_if 10 (;@1;)
                          local.get 9
                          i64.const 1099511627775
                          i64.and
                          local.tee 9
                          i64.const 32
                          i64.shr_u
                          i32.wrap_i64
                          local.set 3
                          local.get 9
                          i64.const 24
                          i64.shr_u
                          i32.wrap_i64
                          local.set 4
                          local.get 9
                          i64.const 16
                          i64.shr_u
                          i32.wrap_i64
                          local.set 5
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                block (result i32)  ;; label = @15
                                  local.get 2
                                  i32.const 8
                                  i32.shr_u
                                  i32.const 255
                                  i32.and
                                  local.tee 2
                                  i32.const 67
                                  i32.ne
                                  if  ;; label = @16
                                    local.get 2
                                    i32.const 224
                                    i32.ne
                                    if  ;; label = @17
                                      local.get 2
                                      i32.const 153
                                      i32.ne
                                      if  ;; label = @18
                                        local.get 2
                                        i32.const 78
                                        i32.ne
                                        local.get 3
                                        i32.const 129
                                        i32.ne
                                        i32.or
                                        local.get 5
                                        i32.const 255
                                        i32.and
                                        i32.const 31
                                        i32.ne
                                        local.get 4
                                        i32.const 255
                                        i32.and
                                        i32.const 45
                                        i32.ne
                                        i32.or
                                        i32.or
                                        br_if 17 (;@1;)
                                        local.get 1
                                        i32.const 168
                                        i32.add
                                        local.get 1
                                        i32.const 32
                                        i32.add
                                        call 11
                                        local.get 1
                                        i32.load offset=168
                                        local.tee 2
                                        i32.eqz
                                        br_if 17 (;@1;)
                                        local.get 1
                                        local.get 1
                                        i64.load offset=172 align=4
                                        i64.store offset=52 align=4
                                        local.get 1
                                        local.get 2
                                        i32.store offset=48
                                        local.get 1
                                        i32.const 168
                                        i32.add
                                        local.get 1
                                        i32.const 32
                                        i32.add
                                        call 14
                                        local.get 1
                                        i32.load offset=168
                                        local.tee 2
                                        i32.eqz
                                        br_if 8 (;@10;)
                                        local.get 1
                                        local.get 1
                                        i64.load offset=172 align=4
                                        i64.store offset=92 align=4
                                        local.get 1
                                        local.get 2
                                        i32.store offset=88
                                        local.get 1
                                        i32.const 168
                                        i32.add
                                        local.get 1
                                        i32.const 32
                                        i32.add
                                        call 14
                                        local.get 1
                                        i32.load offset=168
                                        local.tee 2
                                        i32.eqz
                                        br_if 9 (;@9;)
                                        local.get 1
                                        local.get 1
                                        i64.load offset=172 align=4
                                        i64.store offset=132 align=4
                                        local.get 1
                                        local.get 2
                                        i32.store offset=128
                                        local.get 1
                                        i32.const 168
                                        i32.add
                                        local.get 1
                                        i32.const 32
                                        i32.add
                                        call 11
                                        local.get 1
                                        i32.load offset=168
                                        local.tee 2
                                        i32.eqz
                                        br_if 4 (;@14;)
                                        local.get 1
                                        i64.load offset=172 align=4
                                        local.set 9
                                        local.get 1
                                        i32.const 176
                                        i32.add
                                        local.get 1
                                        i32.const 56
                                        i32.add
                                        i32.load
                                        i32.store
                                        local.get 1
                                        i32.const 216
                                        i32.add
                                        local.get 1
                                        i32.const 96
                                        i32.add
                                        i32.load
                                        i32.store
                                        local.get 1
                                        i32.const 280
                                        i32.add
                                        local.get 1
                                        i32.const 136
                                        i32.add
                                        i32.load
                                        i32.store
                                        local.get 1
                                        local.get 1
                                        i64.load offset=48
                                        i64.store offset=168
                                        local.get 1
                                        local.get 1
                                        i64.load offset=88
                                        i64.store offset=208
                                        local.get 1
                                        local.get 1
                                        i64.load offset=128
                                        i64.store offset=272
                                        i32.const 0
                                        br 3 (;@15;)
                                      end
                                      local.get 3
                                      i32.const 55
                                      i32.ne
                                      local.get 5
                                      i32.const 255
                                      i32.and
                                      i32.const 160
                                      i32.ne
                                      i32.or
                                      local.get 4
                                      i32.const 255
                                      i32.and
                                      i32.const 102
                                      i32.ne
                                      i32.or
                                      br_if 16 (;@1;)
                                      local.get 1
                                      i32.const 168
                                      i32.add
                                      local.get 1
                                      i32.const 32
                                      i32.add
                                      call 11
                                      local.get 1
                                      i32.load offset=168
                                      local.tee 2
                                      i32.eqz
                                      br_if 16 (;@1;)
                                      local.get 1
                                      local.get 1
                                      i64.load offset=172 align=4
                                      i64.store offset=52 align=4
                                      local.get 1
                                      local.get 2
                                      i32.store offset=48
                                      local.get 1
                                      i32.const 168
                                      i32.add
                                      local.get 1
                                      i32.const 32
                                      i32.add
                                      call 14
                                      local.get 1
                                      i32.load offset=168
                                      local.tee 2
                                      i32.eqz
                                      br_if 9 (;@8;)
                                      local.get 1
                                      local.get 1
                                      i64.load offset=172 align=4
                                      i64.store offset=92 align=4
                                      local.get 1
                                      local.get 2
                                      i32.store offset=88
                                      local.get 1
                                      i32.const 168
                                      i32.add
                                      local.get 1
                                      i32.const 32
                                      i32.add
                                      call 14
                                      local.get 1
                                      i32.load offset=168
                                      local.tee 2
                                      i32.eqz
                                      br_if 10 (;@7;)
                                      local.get 1
                                      local.get 1
                                      i64.load offset=172 align=4
                                      i64.store offset=132 align=4
                                      local.get 1
                                      local.get 2
                                      i32.store offset=128
                                      local.get 1
                                      i32.const 168
                                      i32.add
                                      local.get 1
                                      i32.const 32
                                      i32.add
                                      call 11
                                      local.get 1
                                      i32.load offset=168
                                      local.tee 2
                                      i32.eqz
                                      br_if 4 (;@13;)
                                      local.get 1
                                      i64.load offset=172 align=4
                                      local.set 9
                                      local.get 1
                                      i32.const 176
                                      i32.add
                                      local.get 1
                                      i32.const 56
                                      i32.add
                                      i32.load
                                      i32.store
                                      local.get 1
                                      i32.const 216
                                      i32.add
                                      local.get 1
                                      i32.const 96
                                      i32.add
                                      i32.load
                                      i32.store
                                      local.get 1
                                      i32.const 280
                                      i32.add
                                      local.get 1
                                      i32.const 136
                                      i32.add
                                      i32.load
                                      i32.store
                                      local.get 1
                                      local.get 1
                                      i64.load offset=48
                                      i64.store offset=168
                                      local.get 1
                                      local.get 1
                                      i64.load offset=88
                                      i64.store offset=208
                                      local.get 1
                                      local.get 1
                                      i64.load offset=128
                                      i64.store offset=272
                                      i32.const 1
                                      br 2 (;@15;)
                                    end
                                    local.get 3
                                    i32.const 151
                                    i32.ne
                                    local.get 5
                                    i32.const 255
                                    i32.and
                                    i32.const 40
                                    i32.ne
                                    i32.or
                                    local.get 4
                                    i32.const 255
                                    i32.and
                                    i32.const 36
                                    i32.ne
                                    i32.or
                                    br_if 15 (;@1;)
                                    local.get 1
                                    i32.const 168
                                    i32.add
                                    local.get 1
                                    i32.const 32
                                    i32.add
                                    call 11
                                    local.get 1
                                    i32.load offset=168
                                    local.tee 2
                                    i32.eqz
                                    br_if 15 (;@1;)
                                    local.get 1
                                    local.get 1
                                    i64.load offset=172 align=4
                                    i64.store offset=52 align=4
                                    local.get 1
                                    local.get 2
                                    i32.store offset=48
                                    local.get 1
                                    i32.const 168
                                    i32.add
                                    local.get 1
                                    i32.const 32
                                    i32.add
                                    call 14
                                    local.get 1
                                    i32.load offset=168
                                    local.tee 2
                                    i32.eqz
                                    br_if 10 (;@6;)
                                    local.get 1
                                    local.get 1
                                    i64.load offset=172 align=4
                                    i64.store offset=92 align=4
                                    local.get 1
                                    local.get 2
                                    i32.store offset=88
                                    local.get 1
                                    i32.const 168
                                    i32.add
                                    local.get 1
                                    i32.const 32
                                    i32.add
                                    call 14
                                    local.get 1
                                    i32.load offset=168
                                    local.tee 2
                                    i32.eqz
                                    br_if 11 (;@5;)
                                    local.get 1
                                    local.get 1
                                    i64.load offset=172 align=4
                                    i64.store offset=132 align=4
                                    local.get 1
                                    local.get 2
                                    i32.store offset=128
                                    local.get 1
                                    i32.const 168
                                    i32.add
                                    local.get 1
                                    i32.const 32
                                    i32.add
                                    call 11
                                    local.get 1
                                    i32.load offset=168
                                    local.tee 2
                                    i32.eqz
                                    br_if 4 (;@12;)
                                    local.get 1
                                    i64.load offset=172 align=4
                                    local.set 9
                                    local.get 1
                                    i32.const 176
                                    i32.add
                                    local.get 1
                                    i32.const 56
                                    i32.add
                                    i32.load
                                    i32.store
                                    local.get 1
                                    i32.const 216
                                    i32.add
                                    local.get 1
                                    i32.const 96
                                    i32.add
                                    i32.load
                                    i32.store
                                    local.get 1
                                    i32.const 280
                                    i32.add
                                    local.get 1
                                    i32.const 136
                                    i32.add
                                    i32.load
                                    i32.store
                                    local.get 1
                                    local.get 1
                                    i64.load offset=48
                                    i64.store offset=168
                                    local.get 1
                                    local.get 1
                                    i64.load offset=88
                                    i64.store offset=208
                                    local.get 1
                                    local.get 1
                                    i64.load offset=128
                                    i64.store offset=272
                                    i32.const 2
                                    br 1 (;@15;)
                                  end
                                  local.get 3
                                  i32.const 6
                                  i32.ne
                                  local.get 5
                                  i32.const 255
                                  i32.and
                                  i32.const 153
                                  i32.ne
                                  i32.or
                                  local.get 4
                                  i32.const 255
                                  i32.and
                                  i32.const 254
                                  i32.ne
                                  i32.or
                                  br_if 14 (;@1;)
                                  local.get 1
                                  i32.const 168
                                  i32.add
                                  local.get 1
                                  i32.const 32
                                  i32.add
                                  call 11
                                  local.get 1
                                  i32.load offset=168
                                  local.tee 2
                                  i32.eqz
                                  br_if 14 (;@1;)
                                  local.get 1
                                  local.get 1
                                  i64.load offset=172 align=4
                                  i64.store offset=52 align=4
                                  local.get 1
                                  local.get 2
                                  i32.store offset=48
                                  local.get 1
                                  i32.const 168
                                  i32.add
                                  local.get 1
                                  i32.const 32
                                  i32.add
                                  call 14
                                  local.get 1
                                  i32.load offset=168
                                  local.tee 2
                                  i32.eqz
                                  br_if 11 (;@4;)
                                  local.get 1
                                  local.get 1
                                  i64.load offset=172 align=4
                                  i64.store offset=92 align=4
                                  local.get 1
                                  local.get 2
                                  i32.store offset=88
                                  local.get 1
                                  i32.const 168
                                  i32.add
                                  local.get 1
                                  i32.const 32
                                  i32.add
                                  call 14
                                  local.get 1
                                  i32.load offset=168
                                  local.tee 2
                                  i32.eqz
                                  br_if 12 (;@3;)
                                  local.get 1
                                  local.get 1
                                  i64.load offset=172 align=4
                                  i64.store offset=132 align=4
                                  local.get 1
                                  local.get 2
                                  i32.store offset=128
                                  local.get 1
                                  i32.const 168
                                  i32.add
                                  local.get 1
                                  i32.const 32
                                  i32.add
                                  call 11
                                  local.get 1
                                  i32.load offset=168
                                  local.tee 2
                                  i32.eqz
                                  br_if 13 (;@2;)
                                  local.get 1
                                  i64.load offset=172 align=4
                                  local.set 9
                                  local.get 1
                                  i32.const 176
                                  i32.add
                                  local.get 1
                                  i32.const 56
                                  i32.add
                                  i32.load
                                  i32.store
                                  local.get 1
                                  i32.const 216
                                  i32.add
                                  local.get 1
                                  i32.const 96
                                  i32.add
                                  i32.load
                                  i32.store
                                  local.get 1
                                  i32.const 280
                                  i32.add
                                  local.get 1
                                  i32.const 136
                                  i32.add
                                  i32.load
                                  i32.store
                                  local.get 1
                                  local.get 1
                                  i64.load offset=48
                                  i64.store offset=168
                                  local.get 1
                                  local.get 1
                                  i64.load offset=88
                                  i64.store offset=208
                                  local.get 1
                                  local.get 1
                                  i64.load offset=128
                                  i64.store offset=272
                                  i32.const 3
                                end
                                local.set 3
                                local.get 1
                                i32.const 264
                                i32.add
                                local.get 1
                                i32.const 176
                                i32.add
                                i32.load
                                i32.store
                                local.get 1
                                i32.const 248
                                i32.add
                                local.get 1
                                i32.const 216
                                i32.add
                                i32.load
                                i32.store
                                local.get 1
                                i32.const 232
                                i32.add
                                local.get 1
                                i32.const 280
                                i32.add
                                i32.load
                                i32.store
                                local.get 1
                                local.get 1
                                i64.load offset=168
                                i64.store offset=256
                                local.get 1
                                local.get 1
                                i64.load offset=208
                                i64.store offset=240
                                local.get 1
                                local.get 1
                                i64.load offset=272
                                i64.store offset=224
                                i32.const 0
                                local.set 0
                                br 13 (;@1;)
                              end
                              local.get 1
                              i32.const 128
                              i32.add
                              call 26
                              local.get 1
                              i32.const 88
                              i32.add
                              call 26
                              local.get 1
                              i32.const 48
                              i32.add
                              call 4
                              br 12 (;@1;)
                            end
                            local.get 1
                            i32.const 128
                            i32.add
                            call 26
                            local.get 1
                            i32.const 88
                            i32.add
                            call 26
                            local.get 1
                            i32.const 48
                            i32.add
                            call 4
                            br 11 (;@1;)
                          end
                          local.get 1
                          i32.const 128
                          i32.add
                          call 26
                          local.get 1
                          i32.const 88
                          i32.add
                          call 26
                          local.get 1
                          i32.const 48
                          i32.add
                          call 4
                          br 10 (;@1;)
                        end
                        local.get 1
                        i32.const 16384
                        i32.store offset=132
                        local.get 1
                        i32.const 65672
                        i32.store offset=128
                        local.get 1
                        i32.const 128
                        i32.add
                        call 25
                        local.get 1
                        local.get 1
                        i64.load offset=128
                        i64.store offset=168
                        local.get 1
                        i32.const 168
                        i32.add
                        call 9
                        local.set 9
                        local.get 1
                        i32.const 288
                        i32.add
                        global.set 0
                        i32.const 8
                        i32.const 6
                        local.get 9
                        i64.const 1099511627521
                        i64.and
                        i64.const 970968230400
                        i64.eq
                        select
                        return
                      end
                      local.get 1
                      i32.const 48
                      i32.add
                      call 4
                      br 8 (;@1;)
                    end
                    local.get 1
                    i32.const 88
                    i32.add
                    call 26
                    local.get 1
                    i32.const 48
                    i32.add
                    call 4
                    br 7 (;@1;)
                  end
                  local.get 1
                  i32.const 48
                  i32.add
                  call 4
                  br 6 (;@1;)
                end
                local.get 1
                i32.const 88
                i32.add
                call 26
                local.get 1
                i32.const 48
                i32.add
                call 4
                br 5 (;@1;)
              end
              local.get 1
              i32.const 48
              i32.add
              call 4
              br 4 (;@1;)
            end
            local.get 1
            i32.const 88
            i32.add
            call 26
            local.get 1
            i32.const 48
            i32.add
            call 4
            br 3 (;@1;)
          end
          local.get 1
          i32.const 48
          i32.add
          call 4
          br 2 (;@1;)
        end
        local.get 1
        i32.const 88
        i32.add
        call 26
        local.get 1
        i32.const 48
        i32.add
        call 4
        br 1 (;@1;)
      end
      local.get 1
      i32.const 128
      i32.add
      call 26
      local.get 1
      i32.const 88
      i32.add
      call 26
      local.get 1
      i32.const 48
      i32.add
      call 4
    end
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.eqz
        if  ;; label = @3
          local.get 1
          i32.const 40
          i32.add
          local.tee 0
          local.get 1
          i32.const 264
          i32.add
          local.tee 4
          i32.load
          i32.store
          local.get 1
          i32.const 24
          i32.add
          local.tee 5
          local.get 1
          i32.const 248
          i32.add
          local.tee 6
          i32.load
          i32.store
          local.get 1
          i32.const 8
          i32.add
          local.tee 7
          local.get 1
          i32.const 232
          i32.add
          local.tee 8
          i32.load
          i32.store
          local.get 1
          local.get 1
          i64.load offset=256
          i64.store offset=32
          local.get 1
          local.get 1
          i64.load offset=240
          i64.store offset=16
          local.get 1
          local.get 1
          i64.load offset=224
          i64.store
          block  ;; label = @4
            block  ;; label = @5
              local.get 3
              i32.const 1
              i32.sub
              br_table 3 (;@2;) 1 (;@4;) 3 (;@2;) 0 (;@5;)
            end
            local.get 1
            i32.const 56
            i32.add
            local.tee 3
            local.get 0
            i32.load
            i32.store
            local.get 1
            i32.const 68
            i32.add
            local.get 5
            i32.load
            i32.store
            local.get 1
            i32.const 80
            i32.add
            local.get 7
            i32.load
            i32.store
            local.get 1
            local.get 1
            i64.load offset=32
            i64.store offset=48
            local.get 1
            local.get 1
            i64.load offset=16
            i64.store offset=60 align=4
            local.get 1
            local.get 1
            i64.load
            i64.store offset=72
            local.get 1
            i32.const 88
            i32.add
            local.get 1
            i32.const 48
            i32.add
            i32.const 36
            call 81
            drop
            local.get 1
            i32.const 136
            i32.add
            local.get 0
            i32.load
            i32.store
            local.get 1
            i32.const 148
            i32.add
            local.get 1
            i32.const 108
            i32.add
            i32.load
            i32.store
            local.get 1
            i32.const 160
            i32.add
            local.get 1
            i32.const 120
            i32.add
            i32.load
            i32.store
            local.get 1
            local.get 1
            i64.load offset=32
            i64.store offset=128
            local.get 1
            local.get 1
            i64.load offset=100 align=4
            i64.store offset=140 align=4
            local.get 1
            local.get 1
            i64.load offset=112
            i64.store offset=152
            local.get 1
            i32.const 168
            i32.add
            local.get 1
            i32.const 128
            i32.add
            i32.const 36
            call 81
            drop
            local.get 8
            local.get 3
            i32.load
            i32.store
            local.get 1
            local.get 1
            i64.load offset=48
            i64.store offset=224
            local.get 6
            local.get 1
            i32.const 188
            i32.add
            i32.load
            i32.store
            local.get 1
            local.get 1
            i64.load offset=180 align=4
            i64.store offset=240
            local.get 4
            local.get 1
            i32.const 200
            i32.add
            i32.load
            i32.store
            local.get 1
            local.get 1
            i64.load offset=192
            i64.store offset=256
            local.get 1
            local.get 9
            i64.store offset=276 align=4
            local.get 1
            local.get 2
            i32.store offset=272
            local.get 1
            i32.const 208
            i32.add
            i32.const 0
            local.get 1
            i32.const 224
            i32.add
            local.get 1
            i32.const 240
            i32.add
            local.get 1
            i32.const 256
            i32.add
            local.get 1
            i32.const 272
            i32.add
            call 27
            br 3 (;@1;)
          end
          br 1 (;@2;)
        end
        local.get 1
        i32.const 288
        i32.add
        global.set 0
        i32.const 6
        return
      end
      local.get 1
      i32.const 56
      i32.add
      local.tee 3
      local.get 0
      i32.load
      i32.store
      local.get 1
      i32.const 68
      i32.add
      local.get 5
      i32.load
      i32.store
      local.get 1
      i32.const 80
      i32.add
      local.get 7
      i32.load
      i32.store
      local.get 1
      local.get 1
      i64.load offset=32
      i64.store offset=48
      local.get 1
      local.get 1
      i64.load offset=16
      i64.store offset=60 align=4
      local.get 1
      local.get 1
      i64.load
      i64.store offset=72
      local.get 1
      i32.const 88
      i32.add
      local.get 1
      i32.const 48
      i32.add
      i32.const 36
      call 81
      drop
      local.get 1
      i32.const 136
      i32.add
      local.get 0
      i32.load
      i32.store
      local.get 1
      i32.const 148
      i32.add
      local.get 1
      i32.const 108
      i32.add
      i32.load
      i32.store
      local.get 1
      i32.const 160
      i32.add
      local.get 1
      i32.const 120
      i32.add
      i32.load
      i32.store
      local.get 1
      local.get 1
      i64.load offset=32
      i64.store offset=128
      local.get 1
      local.get 1
      i64.load offset=100 align=4
      i64.store offset=140 align=4
      local.get 1
      local.get 1
      i64.load offset=112
      i64.store offset=152
      local.get 1
      i32.const 168
      i32.add
      local.get 1
      i32.const 128
      i32.add
      i32.const 36
      call 81
      drop
      local.get 8
      local.get 3
      i32.load
      i32.store
      local.get 1
      local.get 1
      i64.load offset=48
      i64.store offset=224
      local.get 6
      local.get 1
      i32.const 188
      i32.add
      i32.load
      i32.store
      local.get 1
      local.get 1
      i64.load offset=180 align=4
      i64.store offset=240
      local.get 4
      local.get 1
      i32.const 200
      i32.add
      i32.load
      i32.store
      local.get 1
      local.get 1
      i64.load offset=192
      i64.store offset=256
      local.get 1
      local.get 9
      i64.store offset=276 align=4
      local.get 1
      local.get 2
      i32.store offset=272
      local.get 1
      i32.const 208
      i32.add
      i32.const 1
      local.get 1
      i32.const 224
      i32.add
      local.get 1
      i32.const 240
      i32.add
      local.get 1
      i32.const 256
      i32.add
      local.get 1
      i32.const 272
      i32.add
      call 27
    end
    local.get 1
    i32.load8_u offset=209
    local.set 0
    local.get 1
    i32.load8_u offset=208
    local.set 2
    local.get 1
    i32.const 208
    i32.add
    call 7
    local.get 1
    local.get 2
    i32.eqz
    local.get 0
    i32.const 0
    i32.ne
    i32.and
    i32.store8 offset=272
    local.get 1
    i32.const 272
    i32.add
    call 16
    unreachable)
  (func (;22;) (type 7) (result i32)
    (local i32 i32 i64 i64)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 0
    global.set 0
    local.get 0
    i32.const 16384
    i32.store offset=4
    local.get 0
    i32.const 65672
    i32.store
    local.get 0
    i32.const 16384
    i32.store offset=16
    i32.const 65672
    local.get 0
    i32.const 16
    i32.add
    call 0
    local.get 0
    local.get 0
    i32.load offset=16
    call 23
    local.get 0
    local.get 0
    i64.load
    i64.store offset=8
    local.get 0
    i32.const 24
    i32.add
    local.tee 1
    i64.const 0
    i64.store
    local.get 0
    i64.const 0
    i64.store offset=16
    block (result i32)  ;; label = @1
      i32.const 1
      local.get 0
      i32.const 8
      i32.add
      local.get 0
      i32.const 16
      i32.add
      i32.const 16
      call 24
      br_if 0 (;@1;)
      drop
      local.get 1
      i64.load
      local.set 2
      local.get 0
      i64.load offset=16
      local.set 3
      i32.const 0
    end
    local.get 2
    local.get 3
    i64.or
    i64.eqz
    i32.eqz
    i32.or
    i32.eqz
    if  ;; label = @1
      i32.const 1
      call 21
      local.get 0
      i32.const 32
      i32.add
      global.set 0
      i32.const 255
      i32.and
      i32.const 2
      i32.shl
      i32.const 65536
      i32.add
      i32.load
      return
    end
    unreachable)
  (func (;23;) (type 0) (param i32 i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 0
    i32.load offset=4
    local.set 3
    local.get 0
    i32.const 0
    i32.store offset=4
    local.get 0
    i32.load
    local.set 4
    local.get 0
    i32.const 65572
    i32.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 0
    local.get 1
    local.get 4
    local.get 3
    call 38
    local.get 0
    local.get 2
    i64.load offset=8
    i64.store align=4
    local.get 2
    i32.const 16
    i32.add
    global.set 0)
  (func (;24;) (type 4) (param i32 i32 i32) (result i32)
    block  ;; label = @1
      local.get 0
      i32.load offset=4
      local.get 2
      i32.ge_u
      if (result i32)  ;; label = @2
        local.get 1
        local.get 0
        i32.load
        local.get 2
        call 81
        drop
        local.get 0
        i32.load offset=4
        local.tee 1
        local.get 2
        i32.lt_u
        br_if 1 (;@1;)
        local.get 0
        local.get 1
        local.get 2
        i32.sub
        i32.store offset=4
        local.get 0
        local.get 0
        i32.load
        local.get 2
        i32.add
        i32.store
        i32.const 0
      else
        i32.const 1
      end
      return
    end
    unreachable)
  (func (;25;) (type 2) (param i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    local.get 0
    i32.load offset=4
    i32.store offset=12
    local.get 0
    i32.load
    local.get 1
    i32.const 12
    i32.add
    call 1
    local.get 0
    local.get 1
    i32.load offset=12
    call 23
    local.get 1
    i32.const 16
    i32.add
    global.set 0)
  (func (;26;) (type 2) (param i32)
    (local i32)
    local.get 0
    i32.load offset=4
    local.tee 1
    if  ;; label = @1
      local.get 0
      i32.load
      local.get 1
      call 8
    end)
  (func (;27;) (type 10) (param i32 i32 i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i64 i64 i64 i64)
    global.get 0
    i32.const 464
    i32.sub
    local.tee 6
    global.set 0
    block  ;; label = @1
      local.get 1
      i32.const 15
      i32.and
      if  ;; label = @2
        local.get 0
        i32.const 1
        i32.store8
        local.get 0
        i32.const 12
        i32.add
        i32.const 16
        i32.store
        local.get 0
        i32.const 8
        i32.add
        i32.const 65647
        i32.store
        local.get 0
        i32.const 4
        i32.add
        i32.const 1
        i32.store8
        local.get 5
        call 4
        local.get 4
        call 26
        local.get 3
        call 26
        local.get 2
        call 4
        br 1 (;@1;)
      end
      local.get 6
      i32.const 152
      i32.add
      local.get 2
      i32.const 8
      i32.add
      i32.load
      i32.store
      local.get 6
      local.get 2
      i64.load align=4
      i64.store offset=144
      local.get 6
      i32.const 168
      i32.add
      local.get 3
      i32.const 8
      i32.add
      i32.load
      i32.store
      local.get 6
      local.get 3
      i64.load align=4
      i64.store offset=160
      local.get 6
      i32.const 184
      i32.add
      local.get 4
      i32.const 8
      i32.add
      i32.load
      i32.store
      local.get 6
      local.get 4
      i64.load align=4
      i64.store offset=176
      local.get 6
      i32.const 200
      i32.add
      local.get 5
      i32.const 8
      i32.add
      i32.load
      local.tee 1
      i32.store
      local.get 6
      local.get 5
      i64.load align=4
      i64.store offset=192
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            i32.const 1
            i32.add
            local.tee 3
            local.get 1
            i32.lt_u
            local.tee 2
            br_if 0 (;@4;)
            local.get 3
            local.get 6
            i32.load offset=152
            i32.ne
            br_if 1 (;@3;)
            local.get 2
            br_if 0 (;@4;)
            local.get 6
            i32.load offset=144
            local.tee 2
            i32.load
            local.set 7
            local.get 6
            i32.const 208
            i32.add
            local.tee 4
            local.get 2
            i32.load offset=8
            local.tee 5
            call 41
            local.get 4
            i32.load
            local.get 7
            local.get 5
            call 81
            drop
            local.get 4
            local.get 5
            i32.store offset=8
            local.get 6
            i32.load offset=192
            local.set 4
            local.get 6
            i32.const 240
            i32.add
            i64.const 1
            i64.store
            local.get 6
            i32.const 236
            i32.add
            local.get 2
            local.get 3
            i32.const 12
            i32.mul
            i32.add
            i32.store
            local.get 6
            i32.const 0
            i32.store offset=248
            local.get 6
            local.get 2
            i32.store offset=232
            local.get 6
            local.get 4
            local.get 1
            i32.const 12
            i32.mul
            i32.add
            i32.store offset=228
            local.get 6
            local.get 4
            i32.store offset=224
            local.get 6
            i32.const 448
            i32.add
            i32.const 4
            i32.or
            local.set 1
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            loop  ;; label = @13
                              local.get 6
                              i32.const 136
                              i32.add
                              local.get 6
                              i32.const 224
                              i32.add
                              call 43
                              local.get 6
                              i32.load offset=136
                              local.tee 2
                              i32.eqz
                              if  ;; label = @14
                                local.get 6
                                i32.const 128
                                i32.add
                                local.get 6
                                i32.const 176
                                i32.add
                                i32.const 0
                                i32.const 48
                                call 44
                                local.get 6
                                i32.load offset=132
                                local.set 2
                                local.get 6
                                i32.load offset=128
                                local.set 3
                                local.get 6
                                i32.const 120
                                i32.add
                                local.get 6
                                i32.const 176
                                i32.add
                                i32.const 48
                                i32.const 96
                                call 44
                                local.get 6
                                i32.load offset=124
                                local.set 4
                                local.get 6
                                i32.load offset=120
                                local.set 5
                                local.get 6
                                i32.const 112
                                i32.add
                                local.get 6
                                i32.const 176
                                i32.add
                                i32.const 96
                                i32.const 97
                                call 44
                                local.get 6
                                i32.load offset=116
                                local.set 7
                                local.get 6
                                i32.load offset=112
                                local.set 9
                                local.get 6
                                i32.const 104
                                i32.add
                                local.get 6
                                i32.const 176
                                i32.add
                                i32.const 97
                                i32.const 290
                                call 44
                                local.get 6
                                i32.load offset=108
                                local.set 10
                                local.get 6
                                i32.load offset=104
                                local.set 11
                                local.get 6
                                i32.const 96
                                i32.add
                                local.get 6
                                i32.const 208
                                i32.add
                                i32.const 0
                                i32.const 48
                                call 44
                                local.get 6
                                i32.load offset=100
                                local.set 12
                                local.get 6
                                i32.load offset=96
                                local.set 13
                                local.get 6
                                i32.const 88
                                i32.add
                                local.get 6
                                i32.const 208
                                i32.add
                                i32.const 48
                                i32.const 96
                                call 44
                                local.get 6
                                i32.const 448
                                i32.add
                                local.get 6
                                i32.load offset=88
                                local.get 6
                                i32.load offset=92
                                call 45
                                local.get 6
                                i32.const 408
                                i32.add
                                local.tee 1
                                local.get 6
                                i32.const 460
                                i32.add
                                local.tee 8
                                i32.load
                                i32.store
                                local.get 6
                                local.get 6
                                i64.load offset=452 align=4
                                i64.store offset=400
                                local.get 6
                                i32.load offset=448
                                i32.const 1
                                i32.eq
                                br_if 2 (;@12;)
                                local.get 6
                                i32.const 360
                                i32.add
                                local.get 1
                                i32.load
                                local.tee 14
                                i32.store
                                local.get 6
                                local.get 6
                                i64.load offset=400
                                local.tee 27
                                i64.store offset=352
                                local.get 6
                                i32.const 80
                                i32.add
                                local.get 6
                                i32.const 208
                                i32.add
                                i32.const 96
                                i32.const 97
                                call 44
                                local.get 6
                                i32.load offset=84
                                local.set 15
                                local.get 6
                                i32.load offset=80
                                local.set 16
                                local.get 6
                                i32.const 72
                                i32.add
                                local.get 6
                                i32.const 160
                                i32.add
                                i32.const 0
                                i32.const 193
                                call 44
                                local.get 6
                                i32.load offset=76
                                local.set 17
                                local.get 6
                                i32.load offset=72
                                local.set 18
                                local.get 6
                                i32.const -64
                                i32.sub
                                local.get 6
                                i32.const 176
                                i32.add
                                i32.const 290
                                i32.const 338
                                call 44
                                local.get 6
                                i32.load offset=68
                                local.set 19
                                local.get 6
                                i32.load offset=64
                                local.set 20
                                local.get 6
                                i32.const 56
                                i32.add
                                local.get 6
                                i32.const 176
                                i32.add
                                i32.const 338
                                i32.const 386
                                call 44
                                local.get 6
                                i32.const 448
                                i32.add
                                local.get 6
                                i32.load offset=56
                                local.get 6
                                i32.load offset=60
                                call 45
                                local.get 6
                                i32.const 424
                                i32.add
                                local.get 8
                                i32.load
                                i32.store
                                local.get 6
                                local.get 6
                                i64.load offset=452 align=4
                                i64.store offset=416
                                local.get 6
                                i32.load offset=448
                                i32.const 1
                                i32.eq
                                br_if 5 (;@9;)
                                local.get 6
                                i32.const 376
                                i32.add
                                local.get 6
                                i32.const 424
                                i32.add
                                i32.load
                                local.tee 8
                                i32.store
                                local.get 6
                                local.get 6
                                i64.load offset=416
                                local.tee 28
                                i64.store offset=368
                                local.get 6
                                i32.const 48
                                i32.add
                                local.get 6
                                i32.const 176
                                i32.add
                                i32.const 386
                                i32.const 387
                                call 44
                                local.get 6
                                i32.load offset=52
                                local.set 21
                                local.get 6
                                i32.load offset=48
                                local.set 22
                                local.get 6
                                i32.const 40
                                i32.add
                                local.get 6
                                i32.const 160
                                i32.add
                                i32.const 193
                                i32.const 386
                                call 44
                                local.get 6
                                i32.load offset=44
                                local.set 23
                                local.get 6
                                i32.load offset=40
                                local.set 24
                                local.get 6
                                i32.const 32
                                i32.add
                                local.get 6
                                i32.const 160
                                i32.add
                                i32.const 386
                                i32.const 434
                                call 44
                                local.get 6
                                i32.load offset=36
                                local.set 25
                                local.get 6
                                i32.load offset=32
                                local.set 26
                                local.get 6
                                i32.const 24
                                i32.add
                                local.get 6
                                i32.const 160
                                i32.add
                                i32.const 434
                                i32.const 482
                                call 44
                                local.get 6
                                i32.const 448
                                i32.add
                                local.get 6
                                i32.load offset=24
                                local.get 6
                                i32.load offset=28
                                call 45
                                local.get 6
                                i32.const 440
                                i32.add
                                local.tee 1
                                local.get 6
                                i32.const 460
                                i32.add
                                i32.load
                                i32.store
                                local.get 6
                                local.get 6
                                i64.load offset=452 align=4
                                i64.store offset=432
                                local.get 6
                                i32.load offset=448
                                i32.const 1
                                i32.eq
                                br_if 3 (;@11;)
                                local.get 6
                                i32.const 392
                                i32.add
                                local.get 1
                                i32.load
                                local.tee 1
                                i32.store
                                local.get 6
                                local.get 6
                                i64.load offset=432
                                local.tee 29
                                i64.store offset=384
                                local.get 6
                                i32.const 16
                                i32.add
                                local.get 6
                                i32.const 160
                                i32.add
                                i32.const 482
                                i32.const 483
                                call 44
                                local.get 6
                                i64.load offset=16
                                local.set 30
                                local.get 6
                                i32.const 8
                                i32.add
                                local.get 6
                                i32.const 160
                                i32.add
                                i32.const 483
                                i32.const 676
                                call 44
                                local.get 6
                                i32.const 336
                                i32.add
                                local.get 30
                                i64.store
                                local.get 6
                                i32.const 332
                                i32.add
                                local.get 1
                                i32.store
                                local.get 6
                                i32.const 328
                                i32.add
                                local.get 29
                                i64.store32
                                local.get 6
                                i32.const 324
                                i32.add
                                local.get 25
                                i32.store
                                local.get 6
                                i32.const 316
                                i32.add
                                local.get 23
                                i32.store
                                local.get 6
                                i32.const 312
                                i32.add
                                local.get 24
                                i32.store
                                local.get 6
                                i32.const 308
                                i32.add
                                local.get 21
                                i32.store
                                local.get 6
                                i32.const 304
                                i32.add
                                local.get 22
                                i32.store
                                local.get 6
                                i32.const 300
                                i32.add
                                local.get 8
                                i32.store
                                local.get 6
                                i32.const 296
                                i32.add
                                local.get 28
                                i64.store32
                                local.get 6
                                i32.const 292
                                i32.add
                                local.get 19
                                i32.store
                                local.get 6
                                i32.const 284
                                i32.add
                                local.get 17
                                i32.store
                                local.get 6
                                i32.const 280
                                i32.add
                                local.get 18
                                i32.store
                                local.get 6
                                i32.const 276
                                i32.add
                                local.get 15
                                i32.store
                                local.get 6
                                i32.const 272
                                i32.add
                                local.get 16
                                i32.store
                                local.get 6
                                i32.const 268
                                i32.add
                                local.get 14
                                i32.store
                                local.get 6
                                i32.const 264
                                i32.add
                                local.get 27
                                i64.store32
                                local.get 6
                                i32.const 260
                                i32.add
                                local.get 12
                                i32.store
                                local.get 6
                                i32.const 252
                                i32.add
                                local.get 10
                                i32.store
                                local.get 6
                                i32.const 244
                                i32.add
                                local.get 7
                                i32.store
                                local.get 6
                                i32.const 236
                                i32.add
                                local.get 4
                                i32.store
                                local.get 6
                                i32.const 344
                                i32.add
                                local.get 6
                                i64.load offset=8
                                i64.store
                                local.get 6
                                local.get 26
                                i32.store offset=320
                                local.get 6
                                local.get 20
                                i32.store offset=288
                                local.get 6
                                local.get 13
                                i32.store offset=256
                                local.get 6
                                local.get 11
                                i32.store offset=248
                                local.get 6
                                local.get 9
                                i32.store offset=240
                                local.get 6
                                local.get 5
                                i32.store offset=232
                                local.get 6
                                local.get 2
                                i32.store offset=228
                                local.get 6
                                local.get 3
                                i32.store offset=224
                                local.get 6
                                i32.const 400
                                i32.add
                                i32.const 1160
                                call 41
                                i32.const 128
                                local.set 3
                                local.get 6
                                i32.const 224
                                i32.add
                                local.set 2
                                loop  ;; label = @15
                                  local.get 3
                                  i32.eqz
                                  br_if 5 (;@10;)
                                  local.get 6
                                  i32.const 400
                                  i32.add
                                  local.get 2
                                  i32.load
                                  local.get 2
                                  i32.load offset=4
                                  call 31
                                  local.get 6
                                  i32.const 400
                                  i32.add
                                  local.get 2
                                  i32.load offset=8
                                  local.get 2
                                  i32.load offset=12
                                  call 31
                                  local.get 6
                                  i32.const 400
                                  i32.add
                                  local.get 2
                                  i32.load offset=16
                                  local.get 2
                                  i32.load offset=20
                                  call 31
                                  local.get 6
                                  i32.const 400
                                  i32.add
                                  local.get 2
                                  i32.load offset=24
                                  local.get 2
                                  i32.load offset=28
                                  call 31
                                  local.get 3
                                  i32.const -32
                                  i32.add
                                  local.set 3
                                  local.get 2
                                  i32.const 32
                                  i32.add
                                  local.set 2
                                  br 0 (;@15;)
                                end
                                unreachable
                              end
                              local.get 6
                              i32.load offset=140
                              local.set 3
                              local.get 6
                              i32.const 384
                              i32.add
                              i32.const 129
                              call 41
                              local.get 6
                              i32.const 384
                              i32.add
                              local.get 3
                              i32.load
                              local.get 3
                              i32.load offset=8
                              call 31
                              local.get 6
                              i32.const 384
                              i32.add
                              local.get 2
                              i32.load
                              local.get 2
                              i32.load offset=8
                              call 31
                              local.get 6
                              i32.const 448
                              i32.add
                              i32.const 16777217
                              local.get 6
                              i32.load offset=384
                              local.get 6
                              i32.load offset=392
                              call 50
                              local.get 6
                              i32.const 440
                              i32.add
                              local.tee 2
                              local.get 1
                              i32.const 8
                              i32.add
                              local.tee 3
                              i32.load
                              i32.store
                              local.get 6
                              local.get 1
                              i64.load align=4
                              i64.store offset=432
                              local.get 6
                              i32.load offset=448
                              i32.const 1
                              i32.eq
                              br_if 6 (;@7;)
                              local.get 6
                              i32.const 408
                              i32.add
                              local.tee 4
                              local.get 2
                              i32.load
                              i32.store
                              local.get 6
                              local.get 6
                              i64.load offset=432
                              i64.store offset=400
                              local.get 6
                              i32.const 416
                              i32.add
                              i32.const 194
                              call 41
                              local.get 6
                              i32.const 416
                              i32.add
                              local.get 6
                              i32.load offset=208
                              local.get 6
                              i32.load offset=216
                              call 31
                              local.get 6
                              i32.const 416
                              i32.add
                              local.get 6
                              i32.load offset=400
                              local.get 4
                              i32.load
                              call 31
                              local.get 6
                              i32.const 448
                              i32.add
                              i32.const 16777216
                              local.get 6
                              i32.load offset=416
                              local.get 6
                              i32.load offset=424
                              call 50
                              local.get 2
                              local.get 3
                              i32.load
                              i32.store
                              local.get 6
                              local.get 1
                              i64.load align=4
                              i64.store offset=432
                              local.get 6
                              i32.load offset=448
                              i32.const 1
                              i32.ne
                              if  ;; label = @14
                                local.get 6
                                i32.const 208
                                i32.add
                                call 26
                                local.get 6
                                i32.const 216
                                i32.add
                                local.get 2
                                i32.load
                                i32.store
                                local.get 6
                                local.get 6
                                i64.load offset=432
                                i64.store offset=208
                                local.get 6
                                i32.const 416
                                i32.add
                                call 26
                                local.get 6
                                i32.const 400
                                i32.add
                                call 26
                                local.get 6
                                i32.const 384
                                i32.add
                                call 26
                                br 1 (;@13;)
                              end
                            end
                            local.get 0
                            i32.const 1
                            i32.store8
                            local.get 0
                            i32.const 4
                            i32.add
                            local.get 6
                            i64.load offset=432
                            i64.store align=4
                            local.get 0
                            i32.const 12
                            i32.add
                            local.get 6
                            i32.const 440
                            i32.add
                            i32.load
                            i32.store
                            local.get 6
                            i32.const 416
                            i32.add
                            call 26
                            local.get 6
                            i32.const 400
                            i32.add
                            call 26
                            br 6 (;@6;)
                          end
                          local.get 0
                          i32.const 1
                          i32.store8
                          local.get 0
                          i32.const 4
                          i32.add
                          local.get 6
                          i64.load offset=400
                          i64.store align=4
                          local.get 0
                          i32.const 12
                          i32.add
                          local.get 1
                          i32.load
                          i32.store
                          br 6 (;@5;)
                        end
                        local.get 0
                        i32.const 1
                        i32.store8
                        local.get 0
                        i32.const 4
                        i32.add
                        local.get 6
                        i64.load offset=432
                        i64.store align=4
                        local.get 0
                        i32.const 12
                        i32.add
                        local.get 1
                        i32.load
                        i32.store
                        local.get 6
                        i32.const 368
                        i32.add
                        call 26
                        br 2 (;@8;)
                      end
                      local.get 6
                      i32.const 448
                      i32.add
                      i32.const 16777218
                      local.get 6
                      i32.load offset=400
                      local.get 6
                      i32.load offset=408
                      call 50
                      local.get 6
                      i32.const 440
                      i32.add
                      local.tee 1
                      local.get 6
                      i32.const 460
                      i32.add
                      i32.load
                      i32.store
                      local.get 6
                      local.get 6
                      i64.load offset=452 align=4
                      i64.store offset=432
                      local.get 6
                      i32.load offset=448
                      i32.const 1
                      i32.ne
                      if  ;; label = @10
                        local.get 6
                        i32.const 424
                        i32.add
                        local.get 1
                        i32.load
                        local.tee 1
                        i32.store
                        local.get 6
                        local.get 6
                        i64.load offset=432
                        i64.store offset=416
                        local.get 1
                        i32.eqz
                        br_if 6 (;@4;)
                        local.get 0
                        i32.const 0
                        i32.store8
                        local.get 0
                        local.get 6
                        i32.load offset=416
                        i32.load8_u
                        i32.eqz
                        i32.store8 offset=1
                        local.get 6
                        i32.const 400
                        i32.add
                        call 26
                        local.get 6
                        i32.const 384
                        i32.add
                        call 26
                        local.get 6
                        i32.const 368
                        i32.add
                        call 26
                        local.get 6
                        i32.const 352
                        i32.add
                        call 26
                        local.get 6
                        i32.const 208
                        i32.add
                        call 26
                        local.get 6
                        i32.const 416
                        i32.add
                        call 26
                        local.get 6
                        i32.const 192
                        i32.add
                        call 4
                        local.get 6
                        i32.const 176
                        i32.add
                        call 26
                        local.get 6
                        i32.const 160
                        i32.add
                        call 26
                        local.get 6
                        i32.const 144
                        i32.add
                        call 4
                        br 9 (;@1;)
                      end
                      local.get 0
                      i32.const 1
                      i32.store8
                      local.get 0
                      i32.const 4
                      i32.add
                      local.get 6
                      i64.load offset=432
                      i64.store align=4
                      local.get 0
                      i32.const 12
                      i32.add
                      local.get 1
                      i32.load
                      i32.store
                      local.get 6
                      i32.const 400
                      i32.add
                      call 26
                      local.get 6
                      i32.const 384
                      i32.add
                      call 26
                      local.get 6
                      i32.const 368
                      i32.add
                      call 26
                      local.get 6
                      i32.const 352
                      i32.add
                      call 26
                      local.get 6
                      i32.const 208
                      i32.add
                      call 26
                      br 7 (;@2;)
                    end
                    local.get 0
                    i32.const 1
                    i32.store8
                    local.get 0
                    i32.const 4
                    i32.add
                    local.get 6
                    i64.load offset=416
                    i64.store align=4
                    local.get 0
                    i32.const 12
                    i32.add
                    local.get 6
                    i32.const 424
                    i32.add
                    i32.load
                    i32.store
                  end
                  local.get 6
                  i32.const 352
                  i32.add
                  call 26
                  br 2 (;@5;)
                end
                local.get 0
                i32.const 1
                i32.store8
                local.get 0
                i32.const 4
                i32.add
                local.get 6
                i64.load offset=432
                i64.store align=4
                local.get 0
                i32.const 12
                i32.add
                local.get 6
                i32.const 440
                i32.add
                i32.load
                i32.store
              end
              local.get 6
              i32.const 384
              i32.add
              call 26
            end
            local.get 6
            i32.const 208
            i32.add
            call 26
            br 2 (;@2;)
          end
          unreachable
        end
        local.get 0
        i32.const 1
        i32.store8
        local.get 0
        i32.const 12
        i32.add
        i32.const 27
        i32.store
        local.get 0
        i32.const 8
        i32.add
        i32.const 65572
        i32.store
        local.get 0
        i32.const 4
        i32.add
        i32.const 1
        i32.store8
      end
      local.get 6
      i32.const 192
      i32.add
      call 4
      local.get 6
      i32.const 176
      i32.add
      call 26
      local.get 6
      i32.const 160
      i32.add
      call 26
      local.get 6
      i32.const 144
      i32.add
      call 4
    end
    local.get 6
    i32.const 464
    i32.add
    global.set 0)
  (func (;28;) (type 6) (param i32 i32 i32 i32) (result i32)
    local.get 0
    local.get 1
    local.get 2
    local.get 3
    call 29)
  (func (;29;) (type 6) (param i32 i32 i32 i32) (result i32)
    local.get 3
    local.get 2
    call 36
    local.tee 2
    if  ;; label = @1
      local.get 2
      local.get 0
      local.get 3
      local.get 1
      local.get 1
      local.get 3
      i32.gt_u
      select
      call 81
      drop
      local.get 0
      local.get 1
      call 37
    end
    local.get 2)
  (func (;30;) (type 5) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call 36
    local.tee 1
    if  ;; label = @1
      local.get 1
      local.get 0
      call 80
    end
    local.get 1)
  (func (;31;) (type 1) (param i32 i32 i32)
    local.get 0
    local.get 2
    call 32
    local.get 0
    i32.load
    local.get 0
    i32.load offset=8
    i32.add
    local.get 1
    local.get 2
    call 81
    drop
    local.get 0
    i32.load offset=8
    local.tee 1
    local.get 2
    i32.add
    local.tee 2
    local.get 1
    i32.lt_u
    if  ;; label = @1
      unreachable
    end
    local.get 0
    local.get 2
    i32.store offset=8)
  (func (;32;) (type 0) (param i32 i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load offset=4
        local.tee 3
        local.get 0
        i32.load offset=8
        local.tee 4
        i32.sub
        local.get 1
        i32.lt_u
        if  ;; label = @3
          local.get 1
          local.get 4
          i32.add
          local.tee 1
          local.get 4
          i32.lt_u
          br_if 2 (;@1;)
          local.get 3
          local.get 3
          i32.add
          local.tee 4
          local.get 3
          i32.lt_u
          br_if 2 (;@1;)
          local.get 4
          local.get 1
          local.get 4
          local.get 1
          i32.gt_u
          select
          local.tee 1
          i32.const 8
          local.get 1
          i32.const 8
          i32.gt_u
          select
          local.set 1
          block  ;; label = @4
            local.get 3
            if  ;; label = @5
              local.get 2
              i32.const 24
              i32.add
              i32.const 1
              i32.store
              local.get 2
              local.get 3
              i32.store offset=20
              local.get 2
              local.get 0
              i32.load
              i32.store offset=16
              br 1 (;@4;)
            end
            local.get 2
            i32.const 0
            i32.store offset=16
          end
          local.get 2
          local.get 1
          i32.const 1
          local.get 2
          i32.const 16
          i32.add
          call 15
          local.get 2
          i32.const 8
          i32.add
          i32.load
          local.set 1
          local.get 2
          i32.load
          i32.const 1
          i32.eq
          br_if 1 (;@2;)
          local.get 2
          i32.load offset=4
          local.set 3
          local.get 0
          local.get 1
          i32.store offset=4
          local.get 0
          local.get 3
          i32.store
        end
        local.get 2
        i32.const 32
        i32.add
        global.set 0
        return
      end
      local.get 1
      i32.eqz
      br_if 0 (;@1;)
      unreachable
    end
    unreachable)
  (func (;33;) (type 1) (param i32 i32 i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    i32.const 8
    i32.add
    local.get 1
    local.get 2
    i32.const 0
    call 34
    local.get 0
    local.get 3
    i32.load offset=8
    i32.store
    local.get 0
    local.get 3
    i32.load offset=12
    i32.store offset=4
    local.get 3
    i32.const 16
    i32.add
    global.set 0)
  (func (;34;) (type 3) (param i32 i32 i32 i32)
    block  ;; label = @1
      local.get 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 3
      i32.eqz
      if  ;; label = @2
        local.get 1
        local.get 2
        call 13
        local.set 2
        br 1 (;@1;)
      end
      local.get 1
      local.get 2
      call 30
      local.set 2
    end
    local.get 0
    local.get 1
    i32.store offset=4
    local.get 0
    local.get 2
    i32.store)
  (func (;35;) (type 1) (param i32 i32 i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    global.set 0
    block  ;; label = @1
      local.get 1
      i32.const -1
      i32.gt_s
      if  ;; label = @2
        block (result i32)  ;; label = @3
          local.get 2
          if  ;; label = @4
            local.get 3
            local.get 1
            i32.const 1
            i32.const 1
            call 34
            local.get 3
            i32.load offset=4
            local.set 2
            local.get 3
            i32.load
            br 1 (;@3;)
          end
          local.get 3
          i32.const 8
          i32.add
          local.get 1
          i32.const 1
          call 33
          local.get 3
          i32.load offset=12
          local.set 2
          local.get 3
          i32.load offset=8
        end
        local.tee 1
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        local.get 2
        i32.store offset=4
        local.get 0
        local.get 1
        i32.store
        local.get 3
        i32.const 16
        i32.add
        global.set 0
        return
      end
      unreachable
    end
    unreachable)
  (func (;36;) (type 5) (param i32 i32) (result i32)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 0
    call 76
    local.set 0
    local.get 2
    i32.const 65668
    i32.load
    i32.store offset=12
    block  ;; label = @1
      local.get 0
      local.get 1
      local.get 2
      i32.const 12
      i32.add
      call 78
      local.tee 3
      br_if 0 (;@1;)
      local.get 2
      local.get 0
      local.get 1
      call 77
      i32.const 0
      local.set 3
      local.get 2
      i32.load
      br_if 0 (;@1;)
      local.get 2
      i32.load offset=4
      local.tee 3
      local.get 2
      i32.load offset=12
      i32.store offset=8
      local.get 2
      local.get 3
      i32.store offset=12
      local.get 0
      local.get 1
      local.get 2
      i32.const 12
      i32.add
      call 78
      local.set 3
    end
    i32.const 65668
    local.get 2
    i32.load offset=12
    i32.store
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    local.get 3)
  (func (;37;) (type 0) (param i32 i32)
    (local i32 i32 i32)
    local.get 0
    if  ;; label = @1
      local.get 1
      call 76
      drop
      i32.const 65668
      i32.load
      local.set 3
      local.get 0
      i32.const -8
      i32.add
      local.tee 1
      local.get 1
      i32.load
      local.tee 4
      i32.const -2
      i32.and
      i32.store
      block  ;; label = @2
        block  ;; label = @3
          local.get 4
          i32.const -4
          i32.and
          local.tee 2
          local.get 0
          i32.sub
          local.get 2
          i32.le_u
          if  ;; label = @4
            local.get 0
            i32.const 0
            i32.store
            block  ;; label = @5
              local.get 0
              i32.const -4
              i32.add
              i32.load
              i32.const -4
              i32.and
              local.tee 2
              if  ;; label = @6
                local.get 2
                i32.load8_u
                i32.const 1
                i32.and
                i32.eqz
                br_if 1 (;@5;)
              end
              local.get 4
              i32.const -4
              i32.and
              local.tee 2
              i32.eqz
              br_if 2 (;@3;)
              i32.const 0
              local.get 2
              local.get 4
              i32.const 2
              i32.and
              select
              local.tee 2
              i32.eqz
              br_if 2 (;@3;)
              local.get 2
              i32.load8_u
              i32.const 1
              i32.and
              br_if 2 (;@3;)
              local.get 0
              local.get 2
              i32.load offset=8
              i32.const -4
              i32.and
              i32.store
              local.get 2
              local.get 1
              i32.const 1
              i32.or
              i32.store offset=8
              br 3 (;@2;)
            end
            local.get 1
            call 79
            local.get 2
            i32.load
            local.set 0
            local.get 1
            i32.load8_u
            i32.const 2
            i32.and
            if  ;; label = @5
              local.get 2
              local.get 0
              i32.const 2
              i32.or
              local.tee 0
              i32.store
            end
            local.get 0
            i32.const -4
            i32.and
            local.tee 0
            local.get 2
            i32.sub
            i32.const -8
            i32.add
            local.get 0
            i32.le_u
            br_if 2 (;@2;)
          end
          unreachable
        end
        local.get 0
        local.get 3
        i32.store
        local.get 1
        local.set 3
      end
      i32.const 65668
      local.get 3
      i32.store
    end)
  (func (;38;) (type 9) (param i32 i32 i32 i32 i32)
    block  ;; label = @1
      local.get 1
      local.get 2
      i32.le_u
      if  ;; label = @2
        local.get 4
        local.get 2
        i32.ge_u
        if  ;; label = @3
          local.get 2
          local.get 1
          i32.sub
          local.tee 4
          local.get 2
          i32.gt_u
          br_if 2 (;@1;)
          local.get 0
          local.get 4
          i32.store offset=4
          local.get 0
          local.get 1
          local.get 3
          i32.add
          i32.store
          return
        end
        unreachable
      end
      unreachable
    end
    unreachable)
  (func (;39;) (type 0) (param i32 i32)
    (local i32)
    block  ;; label = @1
      local.get 0
      i32.load offset=8
      local.tee 2
      local.get 0
      i32.load offset=4
      i32.ge_u
      br_if 0 (;@1;)
      local.get 0
      i32.load
      local.get 2
      i32.add
      local.get 1
      i32.store8
      local.get 0
      i32.load offset=8
      local.tee 1
      i32.const 1
      i32.add
      local.tee 2
      local.get 1
      i32.lt_u
      br_if 0 (;@1;)
      local.get 0
      local.get 2
      i32.store offset=8
      return
    end
    unreachable)
  (func (;40;) (type 0) (param i32 i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    local.get 0
    i32.store offset=12
    local.get 1
    local.get 2
    i32.const 12
    i32.add
    i32.const 4
    call 19
    local.get 2
    i32.const 16
    i32.add
    global.set 0)
  (func (;41;) (type 0) (param i32 i32)
    (local i32 i64)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 8
    i32.add
    local.get 1
    i32.const 0
    call 35
    local.get 2
    i64.load offset=8
    local.set 3
    local.get 0
    i32.const 0
    i32.store offset=8
    local.get 0
    local.get 3
    i64.store align=4
    local.get 2
    i32.const 16
    i32.add
    global.set 0)
  (func (;42;) (type 0) (param i32 i32)
    (local i32 i64)
    local.get 1
    i32.load offset=4
    local.tee 2
    if  ;; label = @1
      local.get 2
      i64.extend_i32_u
      i64.const 12
      i64.mul
      local.tee 3
      i64.const 32
      i64.shr_u
      i64.eqz
      if  ;; label = @2
        local.get 0
        local.get 3
        i64.store32 offset=4
        local.get 0
        i32.const 8
        i32.add
        i32.const 4
        i32.store
        local.get 0
        local.get 1
        i32.load
        i32.store
        return
      end
      unreachable
    end
    local.get 0
    i32.const 0
    i32.store)
  (func (;43;) (type 0) (param i32 i32)
    (local i32 i32 i32 i32 i32)
    block  ;; label = @1
      local.get 1
      i32.load
      local.tee 4
      local.get 1
      i32.load offset=4
      i32.eq
      br_if 0 (;@1;)
      local.get 1
      local.get 4
      i32.const 12
      i32.add
      i32.store
      block  ;; label = @2
        local.get 1
        i32.const 16
        i32.add
        i32.load
        local.tee 5
        if  ;; label = @3
          local.get 1
          i32.const 0
          i32.store offset=16
          local.get 1
          local.get 1
          i32.load offset=8
          local.tee 6
          local.get 5
          i32.const 12
          i32.mul
          i32.add
          local.tee 3
          i32.const 12
          i32.add
          local.get 1
          i32.const 12
          i32.add
          i32.load
          local.tee 1
          local.get 1
          local.get 6
          i32.sub
          i32.const 12
          i32.div_u
          local.get 5
          i32.gt_u
          local.tee 1
          select
          i32.store offset=8
          local.get 1
          br_if 1 (;@2;)
          br 2 (;@1;)
        end
        local.get 1
        i32.load offset=8
        local.tee 3
        local.get 1
        i32.const 12
        i32.add
        i32.load
        i32.eq
        br_if 1 (;@1;)
        local.get 1
        local.get 3
        i32.const 12
        i32.add
        i32.store offset=8
      end
      local.get 4
      local.set 2
    end
    local.get 0
    local.get 3
    i32.store offset=4
    local.get 0
    local.get 2
    i32.store)
  (func (;44;) (type 3) (param i32 i32 i32 i32)
    (local i32)
    local.get 2
    local.get 3
    i32.le_u
    if  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.load offset=8
        local.get 3
        i32.ge_u
        if  ;; label = @3
          local.get 3
          local.get 2
          i32.sub
          local.tee 4
          local.get 3
          i32.le_u
          br_if 1 (;@2;)
          unreachable
        end
        unreachable
      end
      local.get 1
      i32.load
      local.set 1
      local.get 0
      local.get 4
      i32.store offset=4
      local.get 0
      local.get 1
      local.get 2
      i32.add
      i32.store
      return
    end
    unreachable)
  (func (;45;) (type 1) (param i32 i32 i32)
    (local i32 i32 i32)
    global.get 0
    i32.const -64
    i32.add
    local.tee 3
    global.set 0
    local.get 3
    i32.const 48
    i32.add
    local.get 1
    local.get 2
    call 46
    local.get 3
    i32.const 32
    i32.add
    local.get 3
    i32.const 48
    i32.add
    call 47
    local.get 3
    i32.const 16
    i32.add
    local.get 3
    i32.const 32
    i32.add
    call 48
    local.get 3
    i32.const 32
    i32.add
    call 49
    local.get 3
    i32.const 48
    i32.add
    local.get 2
    call 41
    local.get 3
    i32.const 8
    i32.add
    local.set 1
    local.get 3
    i32.const 48
    i32.add
    local.tee 4
    i32.load offset=8
    local.get 3
    i32.load offset=24
    local.tee 2
    local.tee 5
    i32.lt_u
    if  ;; label = @1
      unreachable
    end
    local.get 1
    local.get 5
    i32.store offset=4
    local.get 1
    local.get 4
    i32.load
    i32.store
    local.get 3
    i32.load offset=12
    local.get 2
    i32.ne
    if  ;; label = @1
      unreachable
    end
    local.get 3
    i32.load offset=8
    local.get 3
    i32.load offset=16
    local.get 2
    call 81
    drop
    local.get 0
    i32.const 0
    i32.store
    local.get 0
    i32.const 12
    i32.add
    local.get 3
    i32.const 56
    i32.add
    i32.load
    i32.store
    local.get 0
    local.get 3
    i64.load offset=48
    i64.store offset=4 align=4
    local.get 3
    i32.const 16
    i32.add
    call 26
    local.get 3
    i32.const -64
    i32.sub
    global.set 0)
  (func (;46;) (type 1) (param i32 i32 i32)
    (local i32 i32 i32 i32)
    global.get 0
    i32.const 80
    i32.sub
    local.tee 3
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        if  ;; label = @3
          local.get 3
          i32.const 8
          i32.store8 offset=31
          local.get 3
          i32.const 0
          i32.store offset=40
          local.get 3
          i64.const 4
          i64.store offset=32
          local.get 3
          i32.const 48
          i32.add
          local.get 2
          i32.const 4
          call 51
          block  ;; label = @4
            local.get 3
            i32.load offset=52
            i32.const 1
            i32.ne
            if  ;; label = @5
              local.get 3
              i32.const 4
              i32.store offset=56
              local.get 3
              local.get 2
              i32.store offset=52
              local.get 3
              local.get 1
              i32.store offset=48
              local.get 3
              local.get 3
              i32.const 31
              i32.add
              i32.store offset=60
              i32.const 4
              local.set 1
              loop  ;; label = @6
                local.get 3
                i32.const 16
                i32.add
                local.get 3
                i32.const 48
                i32.add
                call 52
                local.get 3
                i32.load offset=16
                local.tee 2
                i32.eqz
                br_if 2 (;@4;)
                local.get 3
                i32.load offset=60
                i32.load8_u
                local.get 2
                local.get 3
                i32.load offset=20
                call 53
                local.set 2
                local.get 3
                i32.load offset=36
                local.get 4
                i32.eq
                if  ;; label = @7
                  local.get 3
                  i32.const -64
                  i32.sub
                  local.get 3
                  i32.load offset=52
                  local.get 3
                  i32.load offset=56
                  call 51
                  local.get 3
                  i32.const 32
                  i32.add
                  i32.const -1
                  local.get 3
                  i32.load offset=64
                  local.tee 1
                  i32.const 1
                  i32.add
                  local.tee 6
                  local.get 6
                  local.get 1
                  i32.lt_u
                  select
                  call 54
                  local.get 3
                  i32.load offset=32
                  local.set 1
                end
                local.get 1
                local.get 5
                i32.add
                local.get 2
                i32.store
                local.get 4
                i32.const 1
                i32.add
                local.tee 2
                local.get 4
                i32.lt_u
                br_if 5 (;@1;)
                local.get 3
                local.get 2
                i32.store offset=40
                local.get 5
                i32.const 4
                i32.add
                local.set 5
                local.get 2
                local.set 4
                br 0 (;@6;)
              end
              unreachable
            end
            local.get 3
            i32.const 32
            i32.add
            local.get 3
            i32.const 56
            i32.add
            i32.load
            call 54
            local.get 3
            i32.load offset=32
            local.get 3
            i32.load offset=40
            local.set 4
            local.get 3
            i32.const 4
            i32.store offset=56
            local.get 3
            local.get 2
            i32.store offset=52
            local.get 3
            local.get 1
            i32.store offset=48
            local.get 4
            i32.const 2
            i32.shl
            i32.add
            local.set 5
            loop  ;; label = @5
              block  ;; label = @6
                local.get 3
                i32.const 8
                i32.add
                local.get 3
                i32.const 48
                i32.add
                call 52
                local.get 3
                i32.load offset=8
                local.tee 1
                i32.eqz
                br_if 0 (;@6;)
                local.get 5
                local.get 3
                i32.load8_u offset=31
                local.get 1
                local.get 3
                i32.load offset=12
                call 53
                i32.store
                local.get 4
                i32.const 1
                i32.add
                local.tee 1
                local.get 4
                i32.lt_u
                br_if 5 (;@1;)
                local.get 5
                i32.const 4
                i32.add
                local.set 5
                local.get 1
                local.set 4
                br 1 (;@5;)
              end
            end
            local.get 3
            local.get 4
            i32.store offset=40
          end
          local.get 3
          i32.const 56
          i32.add
          local.get 3
          i32.const 40
          i32.add
          i32.load
          i32.store
          local.get 3
          local.get 3
          i64.load offset=32
          i64.store offset=48
          local.get 0
          local.get 3
          i32.const 48
          i32.add
          call 55
          br 1 (;@2;)
        end
        local.get 0
        i32.const 0
        i32.store offset=8
        local.get 0
        i64.const 4
        i64.store align=4
      end
      local.get 3
      i32.const 80
      i32.add
      global.set 0
      return
    end
    unreachable)
  (func (;47;) (type 0) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i64 i64 i64)
    global.get 0
    i32.const 176
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 8
    i32.add
    i32.const 65599
    i32.const 48
    call 46
    local.get 2
    i32.const 24
    i32.add
    local.get 2
    i32.const 8
    i32.add
    call 56
    local.get 2
    i32.const 40
    i32.add
    local.get 1
    call 56
    local.get 2
    i32.const -64
    i32.sub
    local.get 2
    i32.const 16
    i32.add
    i32.load
    local.tee 3
    i32.store
    local.get 2
    local.get 2
    i64.load offset=8
    local.tee 12
    i64.store offset=56
    local.get 3
    i32.const 2
    i32.shl
    local.set 4
    local.get 12
    i32.wrap_i64
    local.set 3
    i64.const 0
    local.set 12
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      loop  ;; label = @10
                        local.get 4
                        if  ;; label = @11
                          local.get 13
                          i64.const 63
                          i64.gt_u
                          br_if 2 (;@9;)
                          local.get 4
                          i32.const -4
                          i32.add
                          local.set 4
                          local.get 12
                          local.get 3
                          i64.load32_u
                          local.get 13
                          i64.shl
                          i64.add
                          local.tee 14
                          local.get 12
                          i64.ge_u
                          local.get 13
                          i64.const 32
                          i64.add
                          local.set 13
                          local.get 3
                          i32.const 4
                          i32.add
                          local.set 3
                          local.get 14
                          local.set 12
                          br_if 1 (;@10;)
                          br 10 (;@1;)
                        end
                      end
                      local.get 12
                      i64.const 4294967296
                      i64.lt_u
                      br_if 1 (;@8;)
                    end
                    local.get 2
                    i32.const 80
                    i32.add
                    local.get 2
                    i32.const 48
                    i32.add
                    i32.load
                    i32.store
                    local.get 2
                    local.get 2
                    i64.load offset=40
                    i64.store offset=72
                    local.get 2
                    i32.const 96
                    i32.add
                    local.get 2
                    i32.const -64
                    i32.sub
                    i32.load
                    local.tee 5
                    i32.store
                    local.get 2
                    local.get 2
                    i64.load offset=56
                    i64.store offset=88
                    local.get 5
                    i32.eqz
                    br_if 7 (;@1;)
                    local.get 2
                    i32.load offset=80
                    local.tee 8
                    i32.eqz
                    br_if 1 (;@7;)
                    local.get 5
                    i32.const 1
                    i32.ne
                    if  ;; label = @9
                      local.get 2
                      i32.load offset=88
                      local.set 6
                      i32.const -1
                      local.get 5
                      local.get 8
                      i32.ne
                      local.get 8
                      local.get 5
                      i32.lt_u
                      select
                      local.tee 7
                      br_if 4 (;@5;)
                      local.get 6
                      i32.const -4
                      i32.add
                      local.set 10
                      local.get 5
                      i32.const 2
                      i32.shl
                      local.set 4
                      local.get 8
                      i32.const 2
                      i32.shl
                      local.set 3
                      local.get 2
                      i32.load offset=72
                      i32.const -4
                      i32.add
                      local.set 8
                      loop  ;; label = @10
                        local.get 3
                        i32.eqz
                        br_if 4 (;@6;)
                        local.get 4
                        i32.eqz
                        br_if 6 (;@4;)
                        local.get 3
                        local.get 8
                        i32.add
                        local.set 7
                        local.get 4
                        local.get 10
                        i32.add
                        local.set 9
                        local.get 4
                        i32.const -4
                        i32.add
                        local.set 4
                        local.get 3
                        i32.const -4
                        i32.add
                        local.set 3
                        i32.const -1
                        local.get 7
                        i32.load
                        local.tee 7
                        local.get 9
                        i32.load
                        local.tee 9
                        i32.ne
                        local.get 7
                        local.get 9
                        i32.lt_u
                        select
                        local.tee 7
                        i32.eqz
                        br_if 0 (;@10;)
                      end
                      br 4 (;@5;)
                    end
                    block  ;; label = @9
                      local.get 2
                      i32.load offset=88
                      local.tee 3
                      i32.const 65664
                      i32.ne
                      if  ;; label = @10
                        local.get 3
                        i32.load
                        i32.const 1
                        i32.ne
                        br_if 1 (;@9;)
                      end
                      local.get 2
                      i32.const 140
                      i32.add
                      i32.const 0
                      i32.store
                      local.get 2
                      i32.const 128
                      i32.add
                      local.get 2
                      i32.const 80
                      i32.add
                      i32.load
                      i32.store
                      local.get 2
                      local.get 2
                      i64.load offset=72
                      i64.store offset=120
                      local.get 2
                      i64.const 4
                      i64.store offset=132 align=4
                      local.get 2
                      i32.const 88
                      i32.add
                      call 49
                      br 6 (;@3;)
                    end
                    local.get 2
                    i32.load offset=76
                    local.set 7
                    local.get 2
                    i32.load offset=72
                    local.set 4
                    block  ;; label = @9
                      local.get 3
                      i32.load
                      local.tee 5
                      i32.const 65536
                      i32.ge_u
                      if  ;; label = @10
                        local.get 8
                        i32.const 2
                        i32.shl
                        local.set 6
                        local.get 4
                        i32.const -4
                        i32.add
                        local.set 10
                        i32.const 0
                        local.set 3
                        loop  ;; label = @11
                          local.get 6
                          i32.eqz
                          br_if 2 (;@9;)
                          local.get 2
                          local.get 3
                          local.get 6
                          local.get 10
                          i32.add
                          local.tee 9
                          i32.load
                          local.get 5
                          call 57
                          local.get 2
                          i32.load offset=4
                          local.set 3
                          local.get 9
                          local.get 2
                          i32.load
                          i32.store
                          local.get 6
                          i32.const -4
                          i32.add
                          local.set 6
                          br 0 (;@11;)
                        end
                        unreachable
                      end
                      local.get 8
                      i32.const 2
                      i32.shl
                      local.set 6
                      local.get 4
                      i32.const -4
                      i32.add
                      local.set 10
                      i32.const 0
                      local.set 3
                      loop  ;; label = @10
                        local.get 6
                        i32.eqz
                        br_if 1 (;@9;)
                        local.get 5
                        i32.eqz
                        br_if 9 (;@1;)
                        local.get 6
                        local.get 10
                        i32.add
                        local.tee 9
                        local.get 3
                        i32.const 16
                        i32.shl
                        local.get 9
                        i32.load
                        local.tee 3
                        i32.const 16
                        i32.shr_u
                        i32.or
                        local.tee 9
                        local.get 5
                        i32.rem_u
                        i32.const 16
                        i32.shl
                        local.get 3
                        i32.const 65535
                        i32.and
                        i32.or
                        local.tee 3
                        local.get 5
                        i32.div_u
                        local.get 9
                        local.get 5
                        i32.div_u
                        i32.const 16
                        i32.shl
                        i32.or
                        i32.store
                        local.get 3
                        local.get 5
                        i32.rem_u
                        local.set 3
                        local.get 6
                        i32.const -4
                        i32.add
                        local.set 6
                        br 0 (;@10;)
                      end
                      unreachable
                    end
                    local.get 2
                    local.get 8
                    i32.store offset=128
                    local.get 2
                    local.get 7
                    i32.store offset=124
                    local.get 2
                    local.get 4
                    i32.store offset=120
                    local.get 2
                    i32.const 120
                    i32.add
                    call 58
                    local.get 2
                    i32.const 168
                    i32.add
                    local.get 2
                    i32.load offset=128
                    local.tee 5
                    i32.store
                    local.get 2
                    local.get 2
                    i64.load offset=120
                    local.tee 12
                    i64.store offset=160
                    local.get 2
                    i32.const 152
                    i32.add
                    local.tee 4
                    local.get 5
                    i32.store
                    local.get 2
                    local.get 12
                    i64.store offset=144
                    local.get 2
                    i32.const 0
                    i32.store offset=96
                    block  ;; label = @9
                      local.get 3
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 2
                      i32.const 88
                      i32.add
                      i32.const 0
                      call 59
                      local.get 2
                      i32.load offset=88
                      local.get 2
                      i32.load offset=96
                      local.get 2
                      local.get 3
                      i32.store offset=120
                      local.get 2
                      i32.const 120
                      i32.add
                      i32.const 1
                      call 60
                      local.tee 3
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 2
                      i32.const 88
                      i32.add
                      local.get 3
                      call 59
                    end
                    local.get 2
                    i32.const 128
                    i32.add
                    local.get 4
                    i32.load
                    i32.store
                    local.get 2
                    i32.const 140
                    i32.add
                    local.get 2
                    i32.const 96
                    i32.add
                    i32.load
                    i32.store
                    local.get 2
                    local.get 2
                    i64.load offset=144
                    i64.store offset=120
                    local.get 2
                    local.get 2
                    i64.load offset=88
                    i64.store offset=132 align=4
                    br 5 (;@3;)
                  end
                  local.get 2
                  i32.load offset=48
                  local.set 3
                  local.get 2
                  i32.load offset=40
                  local.set 4
                  block  ;; label = @8
                    local.get 12
                    i32.wrap_i64
                    local.tee 5
                    i32.const 65536
                    i32.ge_u
                    if  ;; label = @9
                      local.get 3
                      i32.const 2
                      i32.shl
                      local.set 3
                      local.get 4
                      i32.const -4
                      i32.add
                      local.set 5
                      i32.const 0
                      local.set 4
                      loop  ;; label = @10
                        local.get 3
                        i32.eqz
                        br_if 2 (;@8;)
                        local.get 3
                        local.get 5
                        i32.add
                        i64.load32_u
                        local.get 4
                        i64.extend_i32_u
                        i64.const 32
                        i64.shl
                        i64.or
                        local.get 12
                        i64.rem_u
                        i32.wrap_i64
                        local.set 4
                        local.get 3
                        i32.const -4
                        i32.add
                        local.set 3
                        br 0 (;@10;)
                      end
                      unreachable
                    end
                    local.get 3
                    i32.const 2
                    i32.shl
                    local.set 3
                    local.get 4
                    i32.const -4
                    i32.add
                    local.set 6
                    i32.const 0
                    local.set 4
                    loop  ;; label = @9
                      local.get 3
                      i32.eqz
                      br_if 1 (;@8;)
                      local.get 5
                      i32.eqz
                      br_if 8 (;@1;)
                      local.get 4
                      i32.const 16
                      i32.shl
                      local.get 3
                      local.get 6
                      i32.add
                      i32.load
                      local.tee 4
                      i32.const 16
                      i32.shr_u
                      i32.or
                      local.get 5
                      i32.rem_u
                      i32.const 16
                      i32.shl
                      local.get 4
                      i32.const 65535
                      i32.and
                      i32.or
                      local.get 5
                      i32.rem_u
                      local.set 4
                      local.get 3
                      i32.const -4
                      i32.add
                      local.set 3
                      br 0 (;@9;)
                    end
                    unreachable
                  end
                  local.get 2
                  i32.const 0
                  i32.store offset=168
                  local.get 2
                  i64.const 4
                  i64.store offset=160
                  local.get 4
                  i64.extend_i32_u
                  local.set 13
                  loop  ;; label = @8
                    local.get 13
                    i64.eqz
                    i32.eqz
                    if  ;; label = @9
                      local.get 2
                      i32.const 160
                      i32.add
                      local.get 13
                      i32.wrap_i64
                      call 59
                      i64.const 0
                      local.set 13
                      br 1 (;@8;)
                    end
                  end
                  local.get 2
                  i32.const 56
                  i32.add
                  call 49
                  local.get 2
                  i32.const 40
                  i32.add
                  call 49
                  br 5 (;@2;)
                end
                local.get 2
                i32.const 136
                i32.add
                i64.const 0
                i64.store
                local.get 2
                i64.const 17179869184
                i64.store offset=128
                local.get 2
                i64.const 4
                i64.store offset=120
                local.get 2
                i32.const 88
                i32.add
                call 49
                local.get 2
                i32.const 72
                i32.add
                call 49
                br 3 (;@3;)
              end
              i32.const -1
              i32.const 0
              local.get 4
              select
              local.set 7
            end
            block  ;; label = @5
              block  ;; label = @6
                local.get 7
                br_table 1 (;@5;) 2 (;@4;) 0 (;@6;)
              end
              local.get 2
              i32.const 140
              i32.add
              local.get 2
              i32.const 80
              i32.add
              i32.load
              i32.store
              local.get 2
              i32.const 0
              i32.store offset=128
              local.get 2
              i64.const 4
              i64.store offset=120
              local.get 2
              local.get 2
              i64.load offset=72
              i64.store offset=132 align=4
              local.get 2
              i32.const 88
              i32.add
              call 49
              br 2 (;@3;)
            end
            local.get 2
            i32.const 0
            i32.store offset=80
            local.get 2
            i32.const 72
            i32.add
            i32.const 1
            call 59
            local.get 2
            i32.const 140
            i32.add
            i32.const 0
            i32.store
            local.get 2
            i32.const 128
            i32.add
            local.get 2
            i32.load offset=80
            i32.store
            local.get 2
            i64.const 4
            i64.store offset=132 align=4
            local.get 2
            local.get 2
            i64.load offset=72
            i64.store offset=120
            local.get 2
            i32.const 88
            i32.add
            call 49
            br 1 (;@3;)
          end
          block (result i32)  ;; label = @4
            local.get 5
            i32.const 2
            i32.shl
            local.get 6
            i32.add
            i32.const -4
            i32.add
            i32.load
            local.tee 9
            i32.clz
            local.tee 6
            if  ;; label = @5
              local.get 2
              i32.const 128
              i32.add
              local.tee 3
              local.get 2
              i32.const 80
              i32.add
              i32.load
              i32.store
              local.get 2
              local.get 2
              i64.load offset=72
              i64.store offset=120
              local.get 2
              i32.const 144
              i32.add
              local.get 2
              i32.const 120
              i32.add
              local.get 6
              call 61
              local.get 3
              local.get 2
              i32.const 96
              i32.add
              i32.load
              i32.store
              local.get 2
              local.get 2
              i64.load offset=88
              i64.store offset=120
              local.get 2
              i32.const 160
              i32.add
              local.get 2
              i32.const 120
              i32.add
              local.get 6
              call 61
              local.get 2
              i32.const 120
              i32.add
              local.get 2
              i32.const 144
              i32.add
              local.get 2
              i32.const 160
              i32.add
              call 62
              local.get 2
              i32.const 160
              i32.add
              call 49
              i32.const 0
              br 1 (;@4;)
            end
            local.get 2
            i32.const 168
            i32.add
            local.get 2
            i32.const 80
            i32.add
            i32.load
            i32.store
            local.get 2
            local.get 2
            i64.load offset=72
            i64.store offset=160
            local.get 2
            i32.const 120
            i32.add
            local.get 2
            i32.const 160
            i32.add
            local.get 2
            i32.const 88
            i32.add
            call 62
            i32.const 1
          end
          local.get 2
          i32.const 112
          i32.add
          local.get 2
          i32.const 128
          i32.add
          local.tee 3
          i32.load
          i32.store
          local.get 2
          local.get 2
          i64.load offset=120
          i64.store offset=104
          local.get 2
          i32.const 136
          i32.add
          i32.load
          local.set 10
          local.get 2
          i32.load offset=132
          local.set 5
          block  ;; label = @4
            block  ;; label = @5
              local.get 2
              i32.const 140
              i32.add
              i32.load
              local.tee 7
              if  ;; label = @6
                local.get 7
                local.get 9
                i32.eqz
                local.tee 8
                i32.le_u
                br_if 1 (;@5;)
                local.get 8
                i32.const 2
                i32.shl
                local.set 4
                local.get 5
                local.set 3
                loop  ;; label = @7
                  local.get 4
                  if  ;; label = @8
                    local.get 4
                    i32.const -4
                    i32.add
                    local.set 4
                    local.get 3
                    i32.const 4
                    i32.add
                    local.set 3
                    br 1 (;@7;)
                  end
                end
                local.get 7
                local.get 8
                i32.sub
                local.tee 3
                i32.eqz
                local.get 9
                i32.or
                i32.eqz
                if  ;; label = @7
                  local.get 5
                  local.get 5
                  local.get 8
                  i32.const 2
                  i32.shl
                  i32.add
                  local.get 3
                  i32.const 2
                  i32.shl
                  call 82
                end
                local.get 2
                local.get 3
                i32.store offset=168
                local.get 2
                local.get 10
                i32.store offset=164
                local.get 2
                local.get 5
                i32.store offset=160
                block  ;; label = @7
                  local.get 6
                  i32.const 31
                  i32.and
                  local.tee 10
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 7
                  i32.const 2
                  i32.shl
                  local.get 8
                  i32.const 2
                  i32.shl
                  i32.sub
                  local.set 4
                  local.get 5
                  i32.const -4
                  i32.add
                  local.set 5
                  i32.const 0
                  local.set 3
                  i32.const 0
                  local.get 6
                  i32.sub
                  i32.const 31
                  i32.and
                  local.set 6
                  loop  ;; label = @8
                    local.get 4
                    i32.eqz
                    br_if 1 (;@7;)
                    local.get 4
                    local.get 5
                    i32.add
                    local.tee 7
                    local.get 3
                    local.get 7
                    i32.load
                    local.tee 7
                    local.get 10
                    i32.shr_u
                    i32.or
                    i32.store
                    local.get 4
                    i32.const -4
                    i32.add
                    local.set 4
                    local.get 7
                    local.get 6
                    i32.shl
                    local.set 3
                    br 0 (;@8;)
                  end
                  unreachable
                end
                local.get 2
                i32.const 128
                i32.add
                local.get 2
                i32.const 168
                i32.add
                i32.load
                i32.store
                local.get 2
                local.get 2
                i64.load offset=160
                i64.store offset=120
                local.get 2
                i32.const 144
                i32.add
                local.get 2
                i32.const 120
                i32.add
                call 55
                br 2 (;@4;)
              end
              local.get 2
              i32.const 132
              i32.add
              i32.const 0
              i32.store
              local.get 3
              local.get 10
              i32.store
              local.get 2
              local.get 5
              i32.store offset=124
              local.get 2
              i32.const 1
              i32.store offset=120
              local.get 2
              i32.const 144
              i32.add
              local.get 2
              i32.const 120
              i32.add
              call 63
              br 1 (;@4;)
            end
            local.get 2
            i32.const 132
            i32.add
            local.get 7
            i32.store
            local.get 2
            i32.const 128
            i32.add
            local.get 10
            i32.store
            local.get 2
            local.get 5
            i32.store offset=124
            local.get 2
            i32.const 1
            i32.store offset=120
            local.get 2
            i32.const 160
            i32.add
            local.get 2
            i32.const 120
            i32.add
            call 63
            local.get 2
            i32.const 152
            i32.add
            i32.const 0
            i32.store
            local.get 2
            local.get 2
            i64.load offset=160
            i64.store offset=144
          end
          local.get 2
          i32.const 128
          i32.add
          local.get 2
          i32.const 112
          i32.add
          i32.load
          i32.store
          local.get 2
          i32.const 140
          i32.add
          local.get 2
          i32.const 152
          i32.add
          i32.load
          i32.store
          local.get 2
          local.get 2
          i64.load offset=104
          i64.store offset=120
          local.get 2
          local.get 2
          i64.load offset=144
          i64.store offset=132 align=4
          i32.eqz
          br_if 0 (;@3;)
          local.get 2
          i32.const 88
          i32.add
          call 49
        end
        local.get 2
        i32.const 168
        i32.add
        local.get 2
        i32.const 140
        i32.add
        i32.load
        i32.store
        local.get 2
        local.get 2
        i64.load offset=132 align=4
        i64.store offset=160
        local.get 2
        i32.const 120
        i32.add
        call 49
      end
      local.get 2
      i32.const 128
      i32.add
      local.get 2
      i32.const 32
      i32.add
      i32.load
      local.tee 10
      i32.store
      local.get 2
      local.get 2
      i64.load offset=24
      local.tee 12
      i64.store offset=120
      local.get 2
      i32.load offset=168
      local.tee 8
      local.get 8
      local.get 10
      local.get 10
      local.get 8
      i32.gt_u
      select
      local.tee 5
      i32.sub
      local.get 8
      i32.gt_u
      br_if 0 (;@1;)
      local.get 12
      i32.wrap_i64
      local.tee 4
      local.get 5
      i32.const 2
      i32.shl
      i32.add
      local.set 7
      i32.const 0
      local.set 9
      local.get 5
      local.set 3
      local.get 2
      i32.load offset=160
      local.tee 11
      local.set 6
      loop  ;; label = @2
        local.get 3
        if  ;; label = @3
          local.get 4
          local.get 4
          i64.load32_u
          local.get 6
          i64.load32_u
          i64.sub
          local.get 9
          i64.extend_i32_u
          i64.const 255
          i64.and
          i64.sub
          local.tee 12
          i64.store32
          local.get 12
          i64.const 63
          i64.shr_u
          i32.wrap_i64
          local.set 9
          local.get 3
          i32.const -1
          i32.add
          local.set 3
          local.get 4
          i32.const 4
          i32.add
          local.set 4
          local.get 6
          i32.const 4
          i32.add
          local.set 6
          br 1 (;@2;)
        else
          local.get 5
          i32.const 2
          i32.shl
          local.set 5
          local.get 9
          if  ;; label = @4
            local.get 10
            i32.const 2
            i32.shl
            local.get 5
            i32.sub
            local.set 4
            loop  ;; label = @5
              local.get 4
              i32.eqz
              br_if 4 (;@1;)
              local.get 7
              local.get 7
              i32.load
              local.tee 3
              i32.const -1
              i32.add
              i32.store
              local.get 4
              i32.const -4
              i32.add
              local.set 4
              local.get 7
              i32.const 4
              i32.add
              local.set 7
              local.get 3
              i32.eqz
              br_if 0 (;@5;)
            end
          end
          local.get 5
          local.get 11
          i32.add
          local.set 3
          local.get 8
          i32.const 2
          i32.shl
          local.get 5
          i32.sub
          local.set 4
          loop  ;; label = @4
            local.get 4
            if  ;; label = @5
              local.get 4
              i32.const -4
              i32.add
              local.set 4
              local.get 3
              i32.load
              local.get 3
              i32.const 4
              i32.add
              local.set 3
              i32.eqz
              br_if 1 (;@4;)
              br 4 (;@1;)
            end
          end
          local.get 2
          i32.const 120
          i32.add
          call 58
          local.get 0
          local.get 2
          i64.load offset=120
          i64.store align=4
          local.get 0
          i32.const 8
          i32.add
          local.get 2
          i32.const 128
          i32.add
          i32.load
          i32.store
          local.get 2
          i32.const 160
          i32.add
          call 49
          local.get 1
          call 49
          local.get 2
          i32.const 176
          i32.add
          global.set 0
          return
        end
        unreachable
      end
      unreachable
    end
    unreachable)
  (func (;48;) (type 0) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32 i64 i64)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.load offset=8
        local.tee 2
        if  ;; label = @3
          block  ;; label = @4
            local.get 2
            i64.extend_i32_u
            i64.const 5
            i64.shl
            local.tee 8
            local.get 1
            i32.load
            local.tee 5
            local.get 2
            i32.const -1
            i32.add
            local.tee 6
            i32.const 2
            i32.shl
            i32.add
            local.tee 7
            i32.load
            i32.clz
            i64.extend_i32_u
            i64.sub
            local.tee 9
            local.get 8
            i64.le_u
            if  ;; label = @5
              local.get 3
              i32.const 8
              i32.add
              local.get 9
              i64.const 7
              i64.and
              i64.const 0
              i64.ne
              i64.extend_i32_u
              local.get 9
              i64.const 3
              i64.shr_u
              i64.add
              local.tee 8
              i64.const 4294967295
              local.get 8
              i64.const 4294967295
              i64.lt_u
              select
              i32.wrap_i64
              i32.const 0
              call 35
              local.get 3
              i64.load offset=8
              local.set 8
              local.get 0
              i32.const 0
              i32.store offset=8
              local.get 0
              local.get 8
              i64.store align=4
              local.get 5
              local.set 2
              br 1 (;@4;)
            end
            unreachable
          end
          loop  ;; label = @4
            local.get 2
            local.get 7
            i32.eq
            if  ;; label = @5
              local.get 5
              local.get 6
              i32.const 2
              i32.shl
              i32.add
              i32.load
              local.set 1
              loop  ;; label = @6
                local.get 1
                i32.eqz
                br_if 4 (;@2;)
                local.get 0
                local.get 1
                call 64
                local.get 1
                i32.const 8
                i32.shr_u
                local.set 1
                br 0 (;@6;)
              end
              unreachable
            else
              local.get 2
              i32.load
              local.set 4
              i32.const 0
              local.set 1
              loop  ;; label = @6
                local.get 1
                i32.const 255
                i32.and
                i32.const 3
                i32.le_u
                if  ;; label = @7
                  local.get 0
                  local.get 4
                  call 64
                  local.get 4
                  i32.const 8
                  i32.shr_u
                  local.set 4
                  local.get 1
                  i32.const 1
                  i32.add
                  local.set 1
                  br 1 (;@6;)
                end
              end
              local.get 2
              i32.const 4
              i32.add
              local.set 2
              br 1 (;@4;)
            end
            unreachable
          end
          unreachable
        end
        i32.const 1
        i32.const 1
        call 13
        local.tee 1
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        i64.const 4294967297
        i64.store offset=4 align=4
        local.get 0
        local.get 1
        i32.store
        local.get 1
        i32.const 0
        i32.store8
      end
      local.get 3
      i32.const 16
      i32.add
      global.set 0
      return
    end
    unreachable)
  (func (;49;) (type 2) (param i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    local.get 0
    call 66
    local.get 1
    i32.load
    local.tee 0
    if  ;; label = @1
      local.get 0
      local.get 1
      i32.load offset=4
      call 71
    end
    local.get 1
    i32.const 16
    i32.add
    global.set 0)
  (func (;50;) (type 3) (param i32 i32 i32 i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 96
    i32.sub
    local.tee 4
    global.set 0
    local.get 4
    i32.const 8
    i32.add
    local.get 3
    i32.const 0
    call 35
    local.get 4
    local.get 4
    i32.load offset=12
    i32.store offset=52
    local.get 4
    local.get 4
    i32.load offset=8
    local.tee 5
    i32.store offset=48
    local.get 5
    local.get 2
    local.get 3
    call 81
    local.set 2
    local.get 4
    local.get 3
    i32.store offset=56
    local.get 4
    i64.const 16384
    i64.store offset=36 align=4
    local.get 4
    i32.const 65672
    i32.store offset=32
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 3
          i32.const 64
          i32.ge_u
          if  ;; label = @4
            local.get 3
            i32.const 16384
            i32.lt_u
            br_if 1 (;@3;)
            local.get 3
            i32.const 1073741824
            i32.lt_u
            br_if 2 (;@2;)
            local.get 4
            i32.const 32
            i32.add
            i32.const 3
            call 39
            local.get 3
            local.get 4
            i32.const 32
            i32.add
            call 40
            br 3 (;@1;)
          end
          local.get 4
          i32.const 32
          i32.add
          local.get 3
          i32.const 2
          i32.shl
          call 39
          br 2 (;@1;)
        end
        local.get 4
        local.get 3
        i32.const 2
        i32.shl
        i32.const 1
        i32.or
        i32.store16 offset=72
        local.get 4
        i32.const 32
        i32.add
        local.get 4
        i32.const 72
        i32.add
        i32.const 2
        call 19
        br 1 (;@1;)
      end
      local.get 3
      i32.const 2
      i32.shl
      i32.const 2
      i32.or
      local.get 4
      i32.const 32
      i32.add
      call 40
    end
    local.get 4
    i32.const 32
    i32.add
    local.get 2
    local.get 3
    call 19
    local.get 4
    i32.load offset=36
    local.tee 5
    local.get 4
    i32.load offset=40
    local.tee 2
    i32.ge_u
    if  ;; label = @1
      local.get 4
      i32.load offset=32
      local.set 3
      local.get 4
      local.get 5
      local.get 2
      i32.sub
      local.tee 5
      i32.store offset=68
      local.get 4
      local.get 2
      local.get 3
      i32.add
      local.tee 6
      i32.store offset=64
      local.get 4
      local.get 5
      i32.store offset=32
      local.get 1
      local.get 3
      local.get 2
      local.get 6
      local.get 4
      i32.const 32
      i32.add
      call 3
      local.set 1
      local.get 4
      i32.const -64
      i32.sub
      local.get 4
      i32.load offset=32
      call 23
      block (result i32)  ;; label = @2
        block (result i32)  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              local.get 1
                              br_table 1 (;@12;) 3 (;@10;) 4 (;@9;) 5 (;@8;) 6 (;@7;) 7 (;@6;) 8 (;@5;) 9 (;@4;) 0 (;@13;) 2 (;@11;)
                            end
                            i32.const 8
                            br 9 (;@3;)
                          end
                          local.get 4
                          local.get 4
                          i64.load offset=64
                          i64.store offset=88
                          local.get 4
                          i32.const 72
                          i32.add
                          local.get 4
                          i32.const 88
                          i32.add
                          call 14
                          i32.const 0
                          local.set 2
                          block (result i32)  ;; label = @12
                            i32.const 1
                            local.get 4
                            i32.load offset=72
                            i32.eqz
                            br_if 0 (;@12;)
                            drop
                            local.get 4
                            i32.const 42
                            i32.add
                            local.get 4
                            i32.const 80
                            i32.add
                            i32.load
                            i32.store align=2
                            local.get 4
                            local.get 4
                            i64.load offset=72
                            i64.store offset=34 align=2
                            i32.const 0
                          end
                          br 9 (;@2;)
                        end
                        i32.const 9
                        br 7 (;@3;)
                      end
                      i32.const 1
                      br 6 (;@3;)
                    end
                    i32.const 2
                    br 5 (;@3;)
                  end
                  i32.const 3
                  br 4 (;@3;)
                end
                i32.const 4
                br 3 (;@3;)
              end
              i32.const 5
              br 2 (;@3;)
            end
            i32.const 6
            br 1 (;@3;)
          end
          i32.const 7
        end
        local.set 2
        i32.const 1
      end
      local.set 3
      local.get 4
      local.get 4
      i64.load offset=32 align=2
      i64.store offset=16
      local.get 4
      local.get 4
      i32.const 38
      i32.add
      i64.load align=2
      i64.store offset=22 align=2
      local.get 0
      block (result i32)  ;; label = @2
        local.get 3
        i32.eqz
        if  ;; label = @3
          local.get 0
          local.get 4
          i64.load offset=18 align=2
          i64.store offset=4 align=2
          local.get 0
          i32.const 12
          i32.add
          local.get 4
          i32.const 26
          i32.add
          i32.load align=2
          i32.store align=2
          local.get 4
          i32.const 48
          i32.add
          call 26
          i32.const 0
          br 1 (;@2;)
        end
        local.get 0
        local.get 2
        i32.store8 offset=5
        local.get 0
        i32.const 2
        i32.store8 offset=4
        local.get 4
        i32.const 48
        i32.add
        call 26
        i32.const 1
      end
      i32.store
      local.get 4
      i32.const 96
      i32.add
      global.set 0
      return
    end
    unreachable)
  (func (;51;) (type 1) (param i32 i32 i32)
    (local i32)
    local.get 1
    if  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        local.get 2
        i32.div_u
        local.set 3
        block  ;; label = @3
          local.get 1
          local.get 2
          i32.rem_u
          i32.eqz
          if  ;; label = @4
            local.get 3
            local.set 1
            br 1 (;@3;)
          end
          local.get 3
          i32.const 1
          i32.add
          local.tee 1
          local.get 3
          i32.lt_u
          br_if 1 (;@2;)
        end
        local.get 0
        i32.const 1
        i32.store offset=4
        local.get 0
        local.get 1
        i32.store
        local.get 0
        i32.const 8
        i32.add
        local.get 1
        i32.store
        return
      end
      unreachable
    end
    local.get 0
    i64.const 4294967296
    i64.store align=4
    local.get 0
    i32.const 8
    i32.add
    i32.const 0
    i32.store)
  (func (;52;) (type 0) (param i32 i32)
    (local i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.load offset=4
        local.tee 2
        i32.eqz
        if  ;; label = @3
          i32.const 0
          local.set 2
          br 1 (;@2;)
        end
        local.get 2
        local.get 1
        i32.load offset=8
        local.tee 3
        local.get 2
        local.get 2
        local.get 3
        i32.gt_u
        select
        local.tee 3
        i32.sub
        local.tee 4
        local.get 2
        i32.gt_u
        br_if 1 (;@1;)
        local.get 1
        i32.load
        local.set 2
        local.get 1
        local.get 4
        i32.store offset=4
        local.get 1
        local.get 2
        local.get 3
        i32.add
        i32.store
      end
      local.get 0
      local.get 3
      i32.store offset=4
      local.get 0
      local.get 2
      i32.store
      return
    end
    unreachable)
  (func (;53;) (type 4) (param i32 i32 i32) (result i32)
    (local i32 i32)
    local.get 1
    i32.const -1
    i32.add
    local.set 3
    local.get 0
    i32.const 255
    i32.and
    local.set 1
    i32.const 0
    local.set 0
    block  ;; label = @1
      loop  ;; label = @2
        local.get 2
        if  ;; label = @3
          local.get 1
          i32.const 32
          i32.ge_u
          br_if 2 (;@1;)
          local.get 2
          local.get 3
          i32.add
          local.get 2
          i32.const -1
          i32.add
          local.set 2
          i32.load8_u
          local.get 0
          local.get 1
          i32.shl
          i32.or
          local.set 0
          br 1 (;@2;)
        end
      end
      local.get 0
      return
    end
    unreachable)
  (func (;54;) (type 0) (param i32 i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 2
    global.set 0
    block (result i32)  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load offset=4
        local.tee 3
        local.get 0
        i32.load offset=8
        local.tee 4
        i32.sub
        local.get 1
        i32.ge_u
        br_if 0 (;@2;)
        local.get 1
        local.get 4
        i32.add
        local.tee 1
        local.get 4
        i32.lt_u
        if  ;; label = @3
          i32.const 0
          local.set 1
          i32.const 1
          br 2 (;@1;)
        end
        local.get 3
        local.get 3
        i32.add
        local.tee 4
        local.get 3
        i32.ge_u
        if  ;; label = @3
          local.get 2
          i32.const 8
          i32.add
          local.get 4
          local.get 1
          local.get 4
          local.get 1
          i32.gt_u
          select
          local.tee 1
          i32.const 4
          local.get 1
          i32.const 4
          i32.gt_u
          select
          call 65
          local.get 2
          i32.load offset=12
          local.set 1
          local.get 2
          i32.load offset=8
          local.set 3
          local.get 2
          i32.const 32
          i32.add
          local.get 0
          call 66
          local.get 2
          i32.const 16
          i32.add
          local.get 3
          local.get 1
          local.get 2
          i32.const 32
          i32.add
          call 15
          local.get 2
          i32.const 24
          i32.add
          i32.load
          local.set 1
          i32.const 1
          local.get 2
          i32.load offset=16
          i32.const 1
          i32.eq
          br_if 2 (;@1;)
          drop
          local.get 0
          local.get 2
          i32.load offset=20
          i32.store
          local.get 0
          local.get 1
          i32.const 2
          i32.shr_u
          i32.store offset=4
          br 1 (;@2;)
        end
        unreachable
      end
      i32.const 0
    end
    local.get 1
    call 67
    local.get 2
    i32.const 48
    i32.add
    global.set 0)
  (func (;55;) (type 0) (param i32 i32)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 8
    i32.add
    local.tee 3
    local.get 1
    i32.const 8
    i32.add
    i32.load
    i32.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store
    local.get 2
    call 58
    local.get 0
    i32.const 8
    i32.add
    local.get 3
    i32.load
    i32.store
    local.get 0
    local.get 2
    i64.load
    i64.store align=4
    local.get 2
    i32.const 16
    i32.add
    global.set 0)
  (func (;56;) (type 0) (param i32 i32)
    (local i32)
    local.get 1
    i32.load
    local.set 2
    local.get 0
    local.get 1
    i32.load offset=8
    local.tee 1
    call 68
    local.get 0
    i32.load
    local.get 2
    local.get 1
    i32.const 2
    i32.shl
    call 81
    drop
    local.get 0
    local.get 1
    i32.store offset=8)
  (func (;57;) (type 3) (param i32 i32 i32 i32)
    (local i64 i64)
    local.get 3
    if  ;; label = @1
      local.get 0
      local.get 2
      i64.extend_i32_u
      local.get 1
      i64.extend_i32_u
      i64.const 32
      i64.shl
      i64.or
      local.tee 4
      local.get 3
      i64.extend_i32_u
      local.tee 5
      i64.rem_u
      i64.store32 offset=4
      local.get 0
      local.get 4
      local.get 5
      i64.div_u
      i64.store32
      return
    end
    unreachable)
  (func (;58;) (type 2) (param i32)
    (local i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 3
    global.set 0
    loop  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load offset=8
        local.tee 1
        i32.eqz
        if  ;; label = @3
          br 1 (;@2;)
        end
        local.get 0
        i32.load
        local.get 1
        i32.const 2
        i32.shl
        i32.add
        i32.const -4
        i32.add
        i32.load
        if  ;; label = @3
          local.get 1
          local.set 2
        else
          local.get 3
          i32.const 8
          i32.add
          local.get 0
          call 72
          br 2 (;@1;)
        end
      end
    end
    local.get 2
    local.get 0
    i32.load offset=4
    local.tee 4
    i32.const 2
    i32.shr_u
    i32.ge_u
    local.get 4
    local.get 1
    i32.le_u
    i32.or
    i32.eqz
    if  ;; label = @1
      local.get 3
      i32.const 16
      i32.add
      local.get 0
      call 66
      block (result i32)  ;; label = @2
        local.get 3
        i32.load offset=16
        local.tee 4
        if  ;; label = @3
          block  ;; label = @4
            local.get 1
            local.get 1
            i32.const 1073741823
            i32.and
            i32.eq
            if  ;; label = @5
              local.get 3
              i32.const 24
              i32.add
              i32.load
              local.set 2
              local.get 3
              i32.load offset=20
              local.set 5
              block  ;; label = @6
                local.get 1
                i32.const 2
                i32.shl
                local.tee 6
                if  ;; label = @7
                  local.get 4
                  local.get 5
                  local.get 2
                  local.get 6
                  call 28
                  local.tee 1
                  i32.eqz
                  br_if 1 (;@6;)
                  br 3 (;@4;)
                end
                local.get 4
                local.get 5
                call 71
                local.get 2
                local.tee 1
                br_if 2 (;@4;)
                i32.const 0
                local.set 2
              end
              i32.const 1
              br 3 (;@2;)
            end
            unreachable
          end
          local.get 0
          local.get 1
          i32.store
          local.get 0
          local.get 6
          i32.const 2
          i32.shr_u
          i32.store offset=4
        end
        i32.const 0
      end
      local.get 2
      call 67
    end
    local.get 3
    i32.const 32
    i32.add
    global.set 0)
  (func (;59;) (type 0) (param i32 i32)
    (local i32)
    local.get 0
    i32.load offset=8
    local.tee 2
    local.get 0
    i32.load offset=4
    i32.eq
    if  ;; label = @1
      local.get 0
      i32.const 1
      call 54
      local.get 0
      i32.load offset=8
      local.set 2
    end
    local.get 0
    i32.load
    local.get 2
    i32.const 2
    i32.shl
    i32.add
    local.get 1
    i32.store
    local.get 0
    i32.load offset=8
    local.tee 1
    i32.const 1
    i32.add
    local.tee 2
    local.get 1
    i32.ge_u
    if  ;; label = @1
      local.get 0
      local.get 2
      i32.store offset=8
      return
    end
    unreachable)
  (func (;60;) (type 6) (param i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i64)
    local.get 1
    local.get 3
    i32.ge_u
    if  ;; label = @1
      local.get 0
      local.get 3
      i32.const 2
      i32.shl
      i32.add
      local.set 6
      local.get 3
      local.set 5
      loop  ;; label = @2
        local.get 5
        if  ;; label = @3
          local.get 0
          local.get 2
          i64.load32_u
          local.get 0
          i64.load32_u
          local.get 4
          i64.extend_i32_u
          i64.const 255
          i64.and
          i64.add
          i64.add
          local.tee 7
          i64.store32
          local.get 5
          i32.const -1
          i32.add
          local.set 5
          local.get 0
          i32.const 4
          i32.add
          local.set 0
          local.get 2
          i32.const 4
          i32.add
          local.set 2
          local.get 7
          i64.const 32
          i64.shr_u
          i32.wrap_i64
          local.set 4
          br 1 (;@2;)
        else
          i32.const 0
          local.set 5
          block  ;; label = @4
            local.get 4
            i32.const 255
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            local.get 1
            i32.const 2
            i32.shl
            local.get 3
            i32.const 2
            i32.shl
            i32.sub
            local.set 0
            loop  ;; label = @5
              local.get 0
              i32.eqz
              if  ;; label = @6
                local.get 4
                local.set 5
                br 2 (;@4;)
              end
              local.get 6
              local.get 6
              i64.load32_u
              local.get 4
              i64.extend_i32_u
              i64.const 255
              i64.and
              i64.add
              local.tee 7
              i64.store32
              local.get 0
              i32.const -4
              i32.add
              local.set 0
              local.get 6
              i32.const 4
              i32.add
              local.set 6
              i32.const 1
              local.set 4
              local.get 7
              i64.const 4294967296
              i64.and
              i64.const 0
              i64.ne
              br_if 0 (;@5;)
            end
          end
          local.get 5
          i32.const 255
          i32.and
          return
        end
        unreachable
      end
      unreachable
    end
    unreachable)
  (func (;61;) (type 1) (param i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i64)
    global.get 0
    i32.const -64
    i32.add
    local.tee 3
    global.set 0
    local.get 1
    i64.load align=4
    local.set 10
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            i32.load offset=8
            local.tee 1
            if  ;; label = @5
              local.get 3
              i32.const 28
              i32.add
              local.get 1
              i32.store
              local.get 3
              i32.const 1
              i32.store offset=16
              local.get 3
              local.get 10
              i64.store offset=20 align=4
              local.get 2
              i32.const 5
              i32.shr_u
              local.tee 7
              i32.eqz
              br_if 1 (;@4;)
              local.get 1
              i32.const 1
              i32.add
              local.tee 5
              local.get 1
              i32.lt_u
              br_if 2 (;@3;)
              local.get 10
              i32.wrap_i64
              local.set 8
              local.get 3
              i32.const 48
              i32.add
              i32.const -1
              local.get 5
              local.get 7
              i32.add
              local.tee 1
              local.get 1
              local.get 7
              i32.lt_u
              select
              call 68
              local.get 3
              i32.const 48
              i32.add
              local.get 7
              call 54
              local.get 7
              i32.const -1
              i32.add
              local.set 6
              local.get 3
              i32.load offset=56
              local.tee 1
              local.get 7
              i32.add
              local.set 9
              local.get 3
              i32.load offset=48
              local.get 1
              i32.const 2
              i32.shl
              i32.add
              local.set 4
              loop  ;; label = @6
                local.get 4
                i32.const 0
                i32.store
                local.get 1
                i32.const 1
                i32.add
                local.tee 5
                local.get 1
                i32.lt_u
                br_if 3 (;@3;)
                local.get 6
                if  ;; label = @7
                  local.get 4
                  i32.const 4
                  i32.add
                  local.set 4
                  local.get 6
                  i32.const -1
                  i32.add
                  local.set 6
                  local.get 5
                  local.set 1
                  br 1 (;@6;)
                end
              end
              local.get 3
              local.get 9
              i32.store offset=56
              local.get 3
              i32.const 48
              i32.add
              local.get 3
              i32.load offset=28
              local.tee 1
              call 54
              local.get 3
              i32.load offset=48
              local.get 3
              i32.load offset=56
              local.tee 5
              i32.const 2
              i32.shl
              i32.add
              local.get 8
              local.get 1
              i32.const 2
              i32.shl
              call 81
              drop
              local.get 1
              local.get 5
              i32.add
              local.tee 1
              local.get 5
              i32.lt_u
              br_if 2 (;@3;)
              local.get 3
              i32.const 40
              i32.add
              local.get 1
              i32.store
              local.get 3
              local.get 3
              i64.load offset=48
              i64.store offset=32
              i32.const 1
              local.set 5
              br 3 (;@2;)
            end
            local.get 3
            i32.const 60
            i32.add
            i32.const 0
            i32.store
            local.get 3
            local.get 10
            i64.store offset=52 align=4
            local.get 3
            i32.const 1
            i32.store offset=48
            local.get 0
            local.get 3
            i32.const 48
            i32.add
            call 63
            br 3 (;@1;)
          end
          local.get 3
          i32.const 56
          i32.add
          local.get 3
          i32.const 24
          i32.add
          i64.load
          i64.store
          local.get 3
          local.get 3
          i64.load offset=16
          i64.store offset=48
          local.get 3
          i32.const 32
          i32.add
          local.get 3
          i32.const 48
          i32.add
          call 63
          br 1 (;@2;)
        end
        unreachable
      end
      block  ;; label = @2
        local.get 2
        i32.const 31
        i32.and
        local.tee 8
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        i32.const 8
        i32.add
        local.get 3
        i32.const 32
        i32.add
        local.get 7
        call 73
        i32.const 0
        local.set 4
        i32.const 0
        local.get 2
        i32.sub
        i32.const 31
        i32.and
        local.set 2
        local.get 3
        i32.load offset=12
        i32.const 2
        i32.shl
        local.set 6
        local.get 3
        i32.load offset=8
        local.set 1
        loop  ;; label = @3
          local.get 6
          if  ;; label = @4
            local.get 1
            local.get 4
            local.get 1
            i32.load
            local.tee 4
            local.get 8
            i32.shl
            i32.or
            i32.store
            local.get 6
            i32.const -4
            i32.add
            local.set 6
            local.get 1
            i32.const 4
            i32.add
            local.set 1
            local.get 4
            local.get 2
            i32.shr_u
            local.set 4
            br 1 (;@3;)
          else
            local.get 4
            i32.eqz
            br_if 2 (;@2;)
            local.get 3
            i32.const 32
            i32.add
            local.get 4
            call 59
          end
        end
      end
      local.get 3
      i32.const 56
      i32.add
      local.get 3
      i32.const 40
      i32.add
      i32.load
      i32.store
      local.get 3
      local.get 3
      i64.load offset=32
      i64.store offset=48
      local.get 0
      local.get 3
      i32.const 48
      i32.add
      call 55
      local.get 5
      i32.eqz
      br_if 0 (;@1;)
      local.get 3
      i32.const 16
      i32.add
      i32.const 4
      i32.or
      call 49
    end
    local.get 3
    i32.const -64
    i32.sub
    global.set 0)
  (func (;62;) (type 1) (param i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i64 i64 i64 i64 i64 i64 i64 i64)
    global.get 0
    i32.const 80
    i32.sub
    local.tee 3
    global.set 0
    block  ;; label = @1
      local.get 2
      i32.load offset=8
      local.tee 4
      i32.eqz
      br_if 0 (;@1;)
      local.get 4
      i32.const -2
      i32.add
      local.tee 7
      local.get 4
      i32.gt_u
      local.get 4
      local.get 7
      i32.le_u
      i32.or
      br_if 0 (;@1;)
      local.get 1
      i32.load offset=8
      local.tee 8
      local.get 4
      i32.sub
      local.tee 5
      local.get 8
      i32.gt_u
      br_if 0 (;@1;)
      local.get 5
      i32.const 1
      i32.add
      local.tee 8
      local.get 5
      i32.lt_u
      br_if 0 (;@1;)
      local.get 2
      i32.load
      local.tee 5
      local.get 4
      i32.const 2
      i32.shl
      i32.add
      i32.const -4
      i32.add
      i32.load
      local.set 11
      local.get 5
      local.get 7
      i32.const 2
      i32.shl
      i32.add
      i64.load32_u
      local.set 20
      local.get 3
      i32.const 40
      i32.add
      local.get 8
      i32.const 1
      call 69
      i64.const 0
      local.get 11
      i64.extend_i32_u
      local.tee 17
      i64.sub
      local.set 21
      local.get 17
      i64.const 32
      i64.shl
      local.set 22
      local.get 3
      i32.load offset=44
      local.set 14
      local.get 3
      i32.load offset=40
      local.set 13
      local.get 8
      local.set 7
      loop  ;; label = @2
        block  ;; label = @3
          local.get 7
          if  ;; label = @4
            local.get 1
            i32.load offset=8
            local.tee 4
            i32.eqz
            br_if 3 (;@1;)
            local.get 4
            i32.const -2
            i32.add
            local.tee 6
            local.get 4
            i32.gt_u
            local.get 4
            local.get 6
            i32.le_u
            i32.or
            br_if 3 (;@1;)
            local.get 1
            i32.load
            local.tee 5
            local.get 4
            i32.const 2
            i32.shl
            i32.add
            i32.const -4
            i32.add
            i32.load
            local.set 4
            local.get 5
            local.get 6
            i32.const 2
            i32.shl
            i32.add
            i64.load32_u
            local.set 16
            block (result i32)  ;; label = @5
              local.get 9
              local.get 11
              i32.lt_u
              if  ;; label = @6
                local.get 3
                i32.const 32
                i32.add
                local.get 9
                local.get 4
                local.get 11
                call 57
                local.get 3
                i64.load32_u offset=36
                local.set 15
                local.get 3
                i32.load offset=32
                br 1 (;@5;)
              end
              local.get 4
              i64.extend_i32_u
              local.get 9
              i64.extend_i32_u
              i64.add
              local.set 15
              i32.const -1
            end
            local.set 4
            local.get 7
            i32.const -1
            i32.add
            local.set 7
            local.get 15
            local.get 21
            i64.add
            local.set 18
            local.get 15
            i64.const 32
            i64.shl
            local.set 15
            loop  ;; label = @5
              local.get 17
              local.get 18
              i64.add
              local.tee 18
              i64.const 4294967296
              i64.ge_u
              if  ;; label = @6
                local.get 4
                i64.extend_i32_u
                local.set 19
                br 3 (;@3;)
              end
              local.get 15
              local.get 16
              i64.or
              local.get 4
              i64.extend_i32_u
              local.tee 19
              local.get 20
              i64.mul
              i64.ge_u
              br_if 2 (;@3;)
              local.get 15
              local.get 22
              i64.add
              local.set 15
              local.get 4
              i32.const -1
              i32.add
              local.get 4
              i32.gt_u
              local.get 4
              i32.const -1
              i32.add
              local.set 4
              i32.eqz
              br_if 0 (;@5;)
            end
            br 3 (;@1;)
          end
          local.get 1
          local.get 9
          call 59
          local.get 1
          call 58
          local.get 3
          local.get 8
          i32.store offset=72
          local.get 3
          local.get 14
          i32.store offset=68
          local.get 3
          local.get 13
          i32.store offset=64
          local.get 3
          i32.const -64
          i32.sub
          call 58
          local.get 3
          i32.const 56
          i32.add
          local.get 3
          i32.load offset=72
          local.tee 2
          i32.store
          local.get 3
          local.get 3
          i64.load offset=64
          local.tee 16
          i64.store offset=48
          local.get 0
          i32.const 8
          i32.add
          local.get 2
          i32.store
          local.get 0
          local.get 16
          i64.store align=4
          local.get 0
          local.get 1
          i64.load align=4
          i64.store offset=12 align=4
          local.get 0
          i32.const 20
          i32.add
          local.get 1
          i32.const 8
          i32.add
          i32.load
          i32.store
          local.get 3
          i32.const 80
          i32.add
          global.set 0
          return
        end
        local.get 3
        i32.const 24
        i32.add
        local.get 1
        local.get 7
        call 73
        local.get 2
        i32.load offset=8
        local.tee 6
        local.get 3
        i32.load offset=28
        local.tee 5
        local.get 5
        local.get 6
        i32.gt_u
        select
        local.set 5
        local.get 2
        i32.load
        local.set 12
        i32.const -1
        local.set 6
        local.get 3
        i32.load offset=24
        local.set 10
        loop  ;; label = @3
          local.get 5
          if  ;; label = @4
            local.get 10
            local.get 6
            i64.extend_i32_u
            local.get 10
            i64.load32_u
            i64.const -4294967296
            i64.or
            i64.add
            local.get 19
            local.get 12
            i64.load32_u
            i64.mul
            i64.sub
            i64.const -4294967295
            i64.add
            local.tee 16
            i64.store32
            local.get 16
            i64.const 32
            i64.shr_u
            i32.wrap_i64
            local.set 6
            local.get 12
            i32.const 4
            i32.add
            local.set 12
            local.get 5
            i32.const -1
            i32.add
            local.set 5
            local.get 10
            i32.const 4
            i32.add
            local.set 10
            br 1 (;@3;)
          end
        end
        block  ;; label = @3
          local.get 9
          local.get 6
          i32.const -1
          i32.xor
          local.tee 6
          i32.ge_u
          if  ;; label = @4
            local.get 4
            local.set 5
            br 1 (;@3;)
          end
          local.get 4
          i32.const -1
          i32.add
          local.tee 5
          local.get 4
          i32.gt_u
          br_if 2 (;@1;)
          local.get 3
          i32.const 16
          i32.add
          local.get 1
          local.get 7
          call 73
          local.get 6
          local.get 3
          i32.load offset=16
          local.get 3
          i32.load offset=20
          local.get 2
          i32.load
          local.get 2
          i32.load offset=8
          call 60
          i32.sub
          local.get 6
          i32.gt_u
          br_if 2 (;@1;)
        end
        local.get 8
        local.get 7
        i32.le_u
        br_if 1 (;@1;)
        local.get 13
        local.get 7
        i32.const 2
        i32.shl
        i32.add
        local.get 5
        i32.store
        local.get 3
        i32.const 8
        i32.add
        local.get 1
        call 72
        local.get 3
        i32.load offset=12
        local.set 9
        local.get 3
        i32.load offset=8
        br_if 0 (;@2;)
      end
    end
    unreachable)
  (func (;63;) (type 0) (param i32 i32)
    (local i32)
    local.get 1
    i32.const 4
    i32.add
    local.set 2
    local.get 1
    i32.load
    i32.const 1
    i32.eq
    if  ;; label = @1
      local.get 0
      local.get 2
      i64.load align=4
      i64.store align=4
      local.get 0
      i32.const 8
      i32.add
      local.get 2
      i32.const 8
      i32.add
      i32.load
      i32.store
      return
    end
    local.get 0
    local.get 2
    i32.load
    call 56)
  (func (;64;) (type 0) (param i32 i32)
    (local i32)
    local.get 0
    i32.load offset=8
    local.tee 2
    local.get 0
    i32.load offset=4
    i32.eq
    if (result i32)  ;; label = @1
      local.get 0
      i32.const 1
      call 32
      local.get 0
      i32.load offset=8
    else
      local.get 2
    end
    local.get 0
    i32.load
    i32.add
    local.get 1
    i32.store8
    local.get 0
    i32.load offset=8
    local.tee 1
    i32.const 1
    i32.add
    local.tee 2
    local.get 1
    i32.ge_u
    if  ;; label = @1
      local.get 0
      local.get 2
      i32.store offset=8
      return
    end
    unreachable)
  (func (;65;) (type 0) (param i32 i32)
    (local i32 i32 i32)
    block  ;; label = @1
      local.get 1
      local.get 1
      i32.const 1073741823
      i32.and
      local.tee 2
      i32.ne
      if  ;; label = @2
        i32.const 0
        local.set 2
        br 1 (;@1;)
      end
      local.get 1
      local.get 2
      i32.eq
      i32.const 2
      i32.shl
      local.tee 2
      local.get 1
      i32.const 2
      i32.shl
      local.tee 3
      i32.add
      i32.const -1
      i32.add
      i32.const 0
      local.get 2
      i32.sub
      local.tee 4
      i32.and
      local.tee 1
      local.get 3
      i32.ge_u
      if  ;; label = @2
        local.get 2
        i32.popcnt
        i32.const 1
        i32.eq
        i32.const 0
        local.get 1
        local.get 4
        i32.le_u
        select
        br_if 1 (;@1;)
        unreachable
      end
      unreachable
    end
    local.get 0
    local.get 2
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store)
  (func (;66;) (type 0) (param i32 i32)
    (local i32)
    local.get 1
    i32.load offset=4
    local.tee 2
    if  ;; label = @1
      local.get 2
      local.get 2
      i32.const 1073741823
      i32.and
      i32.eq
      if  ;; label = @2
        local.get 0
        local.get 2
        i32.const 2
        i32.shl
        i32.store offset=4
        local.get 0
        i32.const 8
        i32.add
        i32.const 4
        i32.store
        local.get 0
        local.get 1
        i32.load
        i32.store
        return
      end
      unreachable
    end
    local.get 0
    i32.const 0
    i32.store)
  (func (;67;) (type 0) (param i32 i32)
    block  ;; label = @1
      local.get 0
      i32.const 1
      i32.eq
      if  ;; label = @2
        local.get 1
        i32.eqz
        br_if 1 (;@1;)
        unreachable
      end
      return
    end
    unreachable)
  (func (;68;) (type 0) (param i32 i32)
    (local i32 i64)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 8
    i32.add
    local.get 1
    i32.const 0
    call 69
    local.get 2
    i64.load offset=8
    local.set 3
    local.get 0
    i32.const 0
    i32.store offset=8
    local.get 0
    local.get 3
    i64.store align=4
    local.get 2
    i32.const 16
    i32.add
    global.set 0)
  (func (;69;) (type 1) (param i32 i32 i32)
    (local i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    i32.const 24
    i32.add
    local.get 1
    call 65
    block  ;; label = @1
      block  ;; label = @2
        local.get 3
        i32.load offset=28
        local.tee 1
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        i32.load offset=24
        local.tee 4
        i32.const -1
        i32.le_s
        br_if 0 (;@2;)
        block (result i32)  ;; label = @3
          local.get 2
          if  ;; label = @4
            local.get 3
            i32.const 8
            i32.add
            local.get 4
            local.get 1
            i32.const 1
            call 70
            local.get 3
            i32.load offset=8
            local.set 2
            local.get 3
            i32.load offset=12
            br 1 (;@3;)
          end
          local.get 3
          i32.const 16
          i32.add
          local.get 4
          local.get 1
          i32.const 0
          call 70
          local.get 3
          i32.load offset=16
          local.set 2
          local.get 3
          i32.load offset=20
        end
        local.set 1
        local.get 2
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        local.get 2
        i32.store
        local.get 0
        local.get 1
        i32.const 2
        i32.shr_u
        i32.store offset=4
        local.get 3
        i32.const 32
        i32.add
        global.set 0
        return
      end
      unreachable
    end
    unreachable)
  (func (;70;) (type 3) (param i32 i32 i32 i32)
    block  ;; label = @1
      local.get 1
      if  ;; label = @2
        local.get 3
        i32.eqz
        if  ;; label = @3
          local.get 1
          local.get 2
          call 13
          local.set 2
          br 2 (;@1;)
        end
        local.get 1
        local.get 2
        call 30
        local.set 2
        br 1 (;@1;)
      end
      i32.const 0
      local.set 1
    end
    local.get 0
    local.get 1
    i32.store offset=4
    local.get 0
    local.get 2
    i32.store)
  (func (;71;) (type 0) (param i32 i32)
    local.get 1
    if  ;; label = @1
      local.get 0
      local.get 1
      call 8
    end)
  (func (;72;) (type 0) (param i32 i32)
    (local i32)
    block (result i32)  ;; label = @1
      i32.const 0
      local.get 1
      i32.load offset=8
      local.tee 2
      i32.eqz
      br_if 0 (;@1;)
      drop
      local.get 1
      local.get 2
      i32.const -1
      i32.add
      local.tee 2
      i32.store offset=8
      local.get 1
      i32.load
      local.get 2
      i32.const 2
      i32.shl
      i32.add
      i32.load
      local.set 2
      i32.const 1
    end
    local.set 1
    local.get 0
    local.get 2
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store)
  (func (;73;) (type 1) (param i32 i32 i32)
    (local i32)
    local.get 1
    i32.load offset=8
    local.tee 3
    local.get 2
    i32.lt_u
    if  ;; label = @1
      unreachable
    end
    local.get 0
    local.get 3
    local.get 2
    i32.sub
    i32.store offset=4
    local.get 0
    local.get 1
    i32.load
    local.get 2
    i32.const 2
    i32.shl
    i32.add
    i32.store)
  (func (;74;) (type 4) (param i32 i32 i32) (result i32)
    (local i32)
    local.get 0
    i32.load16_u offset=4
    local.set 3
    local.get 0
    i32.const 0
    i32.store16 offset=4
    local.get 3
    i32.const 1
    i32.and
    i32.eqz
    if  ;; label = @1
      local.get 0
      i32.load
      local.get 1
      local.get 2
      call 24
      return
    end
    local.get 1
    local.get 3
    i32.const 8
    i32.shr_u
    i32.store8
    local.get 0
    i32.load
    local.get 1
    i32.const 1
    i32.add
    local.get 2
    i32.const -1
    i32.add
    call 24)
  (func (;75;) (type 5) (param i32 i32) (result i32)
    (local i32)
    block  ;; label = @1
      local.get 0
      local.get 1
      i32.add
      local.tee 2
      local.get 0
      i32.lt_u
      br_if 0 (;@1;)
      local.get 2
      i32.const -1
      i32.add
      local.tee 0
      local.get 2
      i32.gt_u
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      i32.div_u
      return
    end
    unreachable)
  (func (;76;) (type 8) (param i32) (result i32)
    local.get 0
    i32.const 4
    call 75)
  (func (;77;) (type 1) (param i32 i32 i32)
    (local i32)
    block  ;; label = @1
      local.get 1
      i32.const 1073741823
      i32.and
      local.get 1
      i32.ne
      br_if 0 (;@1;)
      local.get 2
      i32.const -64
      i32.sub
      local.tee 3
      local.get 2
      i32.lt_u
      local.get 3
      i32.const 536870911
      i32.and
      local.get 3
      i32.ne
      i32.or
      br_if 0 (;@1;)
      local.get 1
      i32.const 2
      i32.shl
      local.tee 1
      local.get 3
      i32.const 3
      i32.shl
      local.tee 2
      local.get 1
      local.get 2
      i32.gt_u
      select
      local.tee 1
      i32.const 8
      i32.add
      local.tee 2
      local.get 1
      i32.lt_u
      br_if 0 (;@1;)
      block (result i32)  ;; label = @2
        i32.const 1
        local.get 2
        i32.const 65536
        call 75
        local.tee 1
        memory.grow
        local.tee 2
        i32.const -1
        i32.eq
        br_if 0 (;@2;)
        drop
        local.get 2
        i32.const 65535
        i32.and
        local.get 2
        i32.ne
        local.get 1
        i32.const 65535
        i32.and
        local.get 1
        i32.ne
        i32.or
        br_if 1 (;@1;)
        local.get 1
        i32.const 16
        i32.shl
        local.tee 3
        i32.const -8
        i32.add
        local.get 3
        i32.gt_u
        br_if 1 (;@1;)
        local.get 2
        i32.const 16
        i32.shl
        local.tee 1
        i64.const 0
        i64.store
        local.get 1
        i32.const 0
        i32.store offset=8
        local.get 1
        local.get 1
        local.get 3
        i32.add
        i32.const 2
        i32.or
        i32.store
        i32.const 0
      end
      local.set 2
      local.get 0
      local.get 1
      i32.store offset=4
      local.get 0
      local.get 2
      i32.store
      return
    end
    unreachable)
  (func (;78;) (type 4) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    local.get 0
    i32.const 2
    i32.shl
    local.set 6
    i32.const 0
    local.get 1
    i32.sub
    local.set 8
    local.get 0
    i32.const 1073741823
    i32.and
    local.get 0
    i32.ne
    local.set 9
    local.get 1
    i32.const -1
    i32.add
    local.tee 10
    local.get 1
    i32.gt_u
    local.set 11
    local.get 2
    i32.load
    local.set 0
    loop  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        local.set 1
        block  ;; label = @3
          loop  ;; label = @4
            block  ;; label = @5
              local.get 1
              i32.load offset=8
              local.tee 0
              i32.const 1
              i32.and
              i32.eqz
              if  ;; label = @6
                local.get 9
                br_if 1 (;@5;)
                local.get 1
                i32.load
                i32.const -4
                i32.and
                local.tee 3
                local.get 1
                i32.sub
                i32.const -8
                i32.add
                local.tee 5
                local.get 3
                i32.gt_u
                br_if 1 (;@5;)
                local.get 5
                local.get 6
                i32.lt_u
                br_if 3 (;@3;)
                local.get 1
                i32.load
                i32.const -4
                i32.and
                local.tee 3
                local.get 6
                i32.sub
                local.tee 12
                local.get 3
                i32.gt_u
                local.get 11
                i32.or
                br_if 1 (;@5;)
                local.get 1
                i32.const 8
                i32.add
                local.tee 5
                i32.const 8
                i32.add
                local.tee 4
                local.get 5
                i32.lt_u
                br_if 1 (;@5;)
                local.get 4
                i32.const -64
                i32.sub
                local.tee 13
                local.get 4
                i32.lt_u
                br_if 1 (;@5;)
                block  ;; label = @7
                  local.get 13
                  local.get 8
                  local.get 12
                  i32.and
                  local.tee 4
                  i32.gt_u
                  if  ;; label = @8
                    local.get 5
                    local.get 10
                    i32.and
                    br_if 5 (;@3;)
                    local.get 2
                    local.get 0
                    i32.const -4
                    i32.and
                    i32.store
                    local.get 1
                    local.get 1
                    i32.load
                    i32.const 1
                    i32.or
                    i32.store
                    local.get 1
                    local.set 0
                    br 1 (;@7;)
                  end
                  local.get 4
                  i32.const -8
                  i32.add
                  local.tee 0
                  local.get 4
                  i32.gt_u
                  br_if 2 (;@5;)
                  local.get 3
                  local.get 0
                  i32.sub
                  local.tee 2
                  local.get 3
                  i32.gt_u
                  local.get 2
                  i32.const -8
                  i32.add
                  local.get 2
                  i32.gt_u
                  i32.or
                  br_if 2 (;@5;)
                  local.get 0
                  i32.const 0
                  i32.store offset=8
                  local.get 0
                  i64.const 0
                  i64.store align=4
                  local.get 0
                  local.get 1
                  i32.load
                  i32.const -4
                  i32.and
                  i32.store
                  block  ;; label = @8
                    local.get 1
                    i32.load
                    local.tee 2
                    i32.const -4
                    i32.and
                    local.tee 3
                    i32.eqz
                    br_if 0 (;@8;)
                    i32.const 0
                    local.get 3
                    local.get 2
                    i32.const 2
                    i32.and
                    select
                    local.tee 2
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 2
                    local.get 2
                    i32.load offset=4
                    i32.const 3
                    i32.and
                    local.get 0
                    i32.or
                    i32.store offset=4
                  end
                  local.get 0
                  local.get 0
                  i32.load offset=4
                  i32.const 3
                  i32.and
                  local.get 1
                  i32.or
                  i32.store offset=4
                  local.get 1
                  local.get 1
                  i32.load offset=8
                  i32.const -2
                  i32.and
                  i32.store offset=8
                  local.get 1
                  local.get 1
                  i32.load
                  local.tee 2
                  i32.const 3
                  i32.and
                  local.get 0
                  i32.or
                  local.tee 3
                  i32.store
                  local.get 2
                  i32.const 2
                  i32.and
                  if  ;; label = @8
                    local.get 1
                    local.get 3
                    i32.const -3
                    i32.and
                    i32.store
                    local.get 0
                    local.get 0
                    i32.load
                    i32.const 2
                    i32.or
                    i32.store
                  end
                  local.get 0
                  local.get 0
                  i32.load
                  i32.const 1
                  i32.or
                  i32.store
                end
                local.get 0
                i32.const 8
                i32.add
                local.set 7
                br 4 (;@2;)
              end
              local.get 1
              local.get 0
              i32.const -2
              i32.and
              i32.store offset=8
              block (result i32)  ;; label = @6
                i32.const 0
                local.get 1
                i32.load offset=4
                i32.const -4
                i32.and
                local.tee 0
                i32.eqz
                br_if 0 (;@6;)
                drop
                i32.const 0
                local.get 0
                local.get 0
                i32.load8_u
                i32.const 1
                i32.and
                select
              end
              local.set 0
              local.get 1
              call 79
              local.get 1
              i32.load8_u
              i32.const 2
              i32.and
              if  ;; label = @6
                local.get 0
                local.get 0
                i32.load
                i32.const 2
                i32.or
                i32.store
              end
              local.get 2
              local.get 0
              i32.store
              local.get 0
              local.tee 1
              i32.load
              i32.const -4
              i32.and
              local.tee 0
              local.get 1
              i32.sub
              i32.const -8
              i32.add
              local.get 0
              i32.le_u
              br_if 1 (;@4;)
            end
          end
          unreachable
        end
        local.get 2
        local.get 0
        i32.store
        br 1 (;@1;)
      end
    end
    local.get 7)
  (func (;79;) (type 2) (param i32)
    (local i32 i32)
    block  ;; label = @1
      local.get 0
      i32.load
      local.tee 1
      i32.const -4
      i32.and
      local.tee 2
      i32.eqz
      br_if 0 (;@1;)
      i32.const 0
      local.get 2
      local.get 1
      i32.const 2
      i32.and
      select
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      local.get 1
      i32.load offset=4
      i32.const 3
      i32.and
      local.get 0
      i32.load offset=4
      i32.const -4
      i32.and
      i32.or
      i32.store offset=4
    end
    local.get 0
    local.get 0
    i32.load offset=4
    local.tee 1
    i32.const -4
    i32.and
    local.tee 2
    if (result i32)  ;; label = @1
      local.get 2
      local.get 2
      i32.load
      i32.const 3
      i32.and
      local.get 0
      i32.load
      i32.const -4
      i32.and
      i32.or
      i32.store
      local.get 0
      i32.load offset=4
    else
      local.get 1
    end
    i32.const 3
    i32.and
    i32.store offset=4
    local.get 0
    local.get 0
    i32.load
    i32.const 3
    i32.and
    i32.store)
  (func (;80;) (type 0) (param i32 i32)
    (local i32)
    loop (result i32)  ;; label = @1
      local.get 1
      local.get 2
      i32.eq
      if (result i32)  ;; label = @2
        local.get 0
      else
        local.get 0
        local.get 2
        i32.add
        i32.const 0
        i32.store8
        local.get 2
        i32.const 1
        i32.add
        local.set 2
        br 1 (;@1;)
      end
    end
    drop)
  (func (;81;) (type 4) (param i32 i32 i32) (result i32)
    (local i32)
    loop (result i32)  ;; label = @1
      local.get 2
      local.get 3
      i32.eq
      if (result i32)  ;; label = @2
        local.get 0
      else
        local.get 0
        local.get 3
        i32.add
        local.get 1
        local.get 3
        i32.add
        i32.load8_u
        i32.store8
        local.get 3
        i32.const 1
        i32.add
        local.set 3
        br 1 (;@1;)
      end
    end)
  (func (;82;) (type 1) (param i32 i32 i32)
    block  ;; label = @1
      local.get 1
      local.get 0
      i32.ge_u
      if  ;; label = @2
        loop  ;; label = @3
          local.get 2
          i32.eqz
          br_if 2 (;@1;)
          local.get 0
          local.get 1
          i32.load8_u
          i32.store8
          local.get 0
          i32.const 1
          i32.add
          local.set 0
          local.get 1
          i32.const 1
          i32.add
          local.set 1
          local.get 2
          i32.const -1
          i32.add
          local.set 2
          br 0 (;@3;)
        end
        unreachable
      end
      local.get 1
      i32.const -1
      i32.add
      local.set 1
      local.get 0
      i32.const -1
      i32.add
      local.set 0
      loop  ;; label = @2
        local.get 2
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        local.get 2
        i32.add
        local.get 1
        local.get 2
        i32.add
        i32.load8_u
        i32.store8
        local.get 2
        i32.const -1
        i32.add
        local.set 2
        br 0 (;@2;)
      end
      unreachable
    end)
  (table (;0;) 1 1 funcref)
  (global (;0;) (mut i32) (i32.const 65536))
  (export "deploy" (func 20))
  (export "call" (func 22))
  (data (;0;) (i32.const 65536) "\01\00\00\00\02\00\00\00\03\00\00\00\04\00\00\00\05\00\00\00\06\00\00\00\07\00\00\00\08\00\00\00\00\00\00\00Verifying key was malformed\01\00\00\00\00\c0\08\85\00\00\000D]\0b\17\00H\09\ba/b\f3\1e\8f\13\f5\00\f3\d9\22\1a;I\a1l\c0\05;\c6\ea\10\c5\17F:\ae\01Invalid curve id\00\01"))

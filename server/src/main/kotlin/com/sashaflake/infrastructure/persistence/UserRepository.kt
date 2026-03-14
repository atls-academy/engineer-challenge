package com.sashaflake.infrastructure.persistence

import com.sashaflake.domain.user.User
import kotlinx.coroutines.Dispatchers
import org.jetbrains.exposed.sql.*
import org.jetbrains.exposed.sql.SqlExpressionBuilder.eq
import org.jetbrains.exposed.sql.transactions.experimental.newSuspendedTransaction
import org.jetbrains.exposed.sql.transactions.transaction

class UserRepository(database: Database) {

    private object Users : Table() {
        val id = integer("id").autoIncrement()
        val name = varchar("name", 50)
        val age = integer("age")
        override val primaryKey = PrimaryKey(id)
    }

    init {
        transaction(database) { SchemaUtils.create(Users) }
    }

    suspend fun create(user: User): Int = dbQuery {
        Users.insert {
            it[name] = user.name
            it[age] = user.age
        }[Users.id]
    }

    suspend fun read(id: Int): User? = dbQuery {
        Users.selectAll()
            .where { Users.id eq id }
            .map { User(it[Users.name], it[Users.age]) }
            .singleOrNull()
    }

    suspend fun update(id: Int, user: User) = dbQuery {
        Users.update({ Users.id eq id }) {
            it[name] = user.name
            it[age] = user.age
        }
    }

    suspend fun delete(id: Int) = dbQuery {
        Users.deleteWhere { Users.id eq id }
    }

    private suspend fun <T> dbQuery(block: suspend () -> T): T =
        newSuspendedTransaction(Dispatchers.IO) { block() }
}
